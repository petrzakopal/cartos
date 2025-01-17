use common::{types::card::CardType, utils::utf_32_to_8_conversion::Utf32To8};
use pcsc::*;
use tokio::time::{sleep, Duration};
use tracing::warn;
use tracing_log::log::{debug, error, info};

const MIFARE_SERIAL_NAME_BYTE_LENGTH: usize = 6;
const NTAG215_SERIAL_NAME_BYTE_LENGHT: usize = 16; // Acutally 8 needed
const MIFARE_SERIAL_NUMBER_APDU: &[u8; 5] = b"\xFF\xCA\x00\x00\x00";
const NTAG215_SERIAL_NUMBER_APDU: &[u8; 2] = b"\x30\x00";
const MIFAREDESFIREEV18K_SERIAL_NAME_BYTE_LENGTH: usize = 9; // Acutally 8 needed

/// Read serial number function for Mifare, Ntag215 or Unknown CardType (when selected Unknown
/// first it checks for Ntag215 card and when no enough data is read for the serial number
/// it automatically checks the card for Mifare serial number)
pub fn read_serial_number(
    ctx: &Context,
    mut readers_buf: [u8; 2048],
    card_type: CardType,
) -> String {
    let readers = ctx
        .list_readers(&mut readers_buf)
        .expect("failed to list readers")
        .collect::<Vec<_>>();
    let mut serial_number_formatted_string: String = String::default();

    let _ = match ctx.connect(readers[0], ShareMode::Shared, Protocols::ANY) {
        Ok(mut connection) => {
            let tx = connection
                .transaction()
                .expect("failed to begin card transaction");

            let (names_len, _atr_len) = tx.status2_len().expect("failed to get the status length");
            let mut names_buf = vec![0; names_len];
            let mut atr_buf = [0; MAX_ATR_SIZE];

            let status = tx
                .status2(&mut names_buf, &mut atr_buf)
                .expect("failed to get card status");

            info!("Status from status: {:?}", status.status());

            println!(
                "Reader names from status: {:?}",
                status.reader_names().collect::<Vec<_>>()
            );

            let mut rapdu_buf = [0; MAX_BUFFER_SIZE];

            match card_type {
                CardType::Mifare => {
                    let apdu = MIFARE_SERIAL_NUMBER_APDU;
                    let rapdu = tx
                        .transmit(apdu, &mut rapdu_buf)
                        .expect("failed to transmit APDU to card");
                    let mut serial_number: Vec<u8> = Vec::new();
                    process_mifare_serial_number_type(
                        serial_number,
                        rapdu,
                        &mut serial_number_formatted_string,
                    );
                }
                CardType::Ntag215 => {
                    let apdu = NTAG215_SERIAL_NUMBER_APDU;

                    let rapdu = tx
                        .transmit(apdu, &mut rapdu_buf)
                        .expect("failed to transmit APDU to card");
                    let mut serial_number: Vec<u8> = Vec::new();

                    process_ntag215_serial_number_type(
                        serial_number,
                        rapdu,
                        &mut serial_number_formatted_string,
                    );
                }
                CardType::Unknown => {
                    // Try NTAG215 first, if not enough values do Mifare

                    let apdu = NTAG215_SERIAL_NUMBER_APDU;

                    let rapdu = tx
                        .transmit(apdu, &mut rapdu_buf)
                        .expect("failed to transmit APDU to card");
                    let mut serial_number: Vec<u8> = Vec::new();

                    if rapdu.len() == NTAG215_SERIAL_NAME_BYTE_LENGHT {
                        process_ntag215_serial_number_type(
                            serial_number,
                            rapdu,
                            &mut serial_number_formatted_string,
                        );
                    } else {
                        warn!("This is not an NTAG215 card or the process of reading data from card has been interrupted. The data is not a serial number, changing CardType to Mifare and trying to read the serial number again.");

                        let apdu = MIFARE_SERIAL_NUMBER_APDU;
                        let rapdu = tx
                            .transmit(apdu, &mut rapdu_buf)
                            .expect("failed to transmit APDU to card");
                        let mut serial_number: Vec<u8> = Vec::new();
                        process_mifare_serial_number_type(
                            serial_number,
                            rapdu,
                            &mut serial_number_formatted_string,
                        );
                    }
                }
            }

            // Leave card
            match tx.end(Disposition::LeaveCard) {
                Ok(_val) => {
                    info!("Successfully ended transmission of the data.");
                }
                Err(_e) => {
                    error!("Failed to end the transaction of the data.");
                }
            }
        }
        Err(err) => {
            error!("Failed to connect to card: {:?}", err);
            serial_number_formatted_string = String::from("error reading serial number");
            // Optionally add a delay before retrying to avoid busy looping
            let _ = sleep(Duration::from_millis(100));
        }
    };
    return serial_number_formatted_string;
}

/// Processes mifare serial number based on number of received data
/// and assigns the string to the variable which can be used in the calling
/// function
fn process_mifare_serial_number_type(
    mut serial_number: Vec<u8>,
    rapdu: &[u8],
    serial_number_formatted_string: &mut String,
) {
    if (rapdu.len() == MIFARE_SERIAL_NAME_BYTE_LENGTH)
        || (rapdu.len() == MIFAREDESFIREEV18K_SERIAL_NAME_BYTE_LENGTH)
    {
        info!("The read file is of type Mifare");
        if rapdu.len() == MIFARE_SERIAL_NAME_BYTE_LENGTH {
            debug!("Classic MIFARE.");

            serial_number.extend_from_slice(&[rapdu[0], rapdu[1], rapdu[2], rapdu[3]]);
        } else if rapdu.len() == MIFAREDESFIREEV18K_SERIAL_NAME_BYTE_LENGTH {
            debug!("DESFire EV1 8k MIFARE.");
            serial_number.extend_from_slice(&[
                rapdu[0], rapdu[1], rapdu[2], rapdu[3], rapdu[4], rapdu[5], rapdu[6],
            ]);
        }
        let data = Utf32To8(serial_number.as_ref());
        info!("Serial number raw MIFARE: {}", data.display_raw_string());
        *serial_number_formatted_string = data.serial_number_format();
        info!(
            "Serial number formatted string MIFARE: {}",
            serial_number_formatted_string
        );
        for value in rapdu {
            debug!("Serial number value MIFARE: {:x}", value);
        }
    } else {
        error!("The reading of serial name bytes has been interrupted or there are not enough bytes read.");
    }
}

/// Processes ntag 215 serial number based on number of received data
/// and assigns the string to the variable which can be used in the calling
/// function
fn process_ntag215_serial_number_type(
    mut serial_number: Vec<u8>,
    rapdu: &[u8],
    serial_number_formatted_string: &mut String,
) {
    if rapdu.len() == NTAG215_SERIAL_NAME_BYTE_LENGHT {
        info!("The read file is of type NTAG215");
        serial_number.extend_from_slice(&[
            rapdu[0], rapdu[1], rapdu[2], rapdu[4], rapdu[5], rapdu[6], rapdu[7],
        ]);
        let data = Utf32To8(serial_number.as_ref());
        info!("Serial number raw NTAG: {}", data.display_raw_string());
        *serial_number_formatted_string = data.serial_number_format();
        info!(
            "Serial number formatted string NTAG: {}",
            serial_number_formatted_string
        );
        for value in rapdu {
            debug!("Serial number value NTAG215: {:x}", value);
        }
    } else {
        warn!("This is not an NTAG215 card or the process of reading data from card has been interrupted. The data is not a serial number, changing CardType to Mifare and trying to read the serial number again.");
    }
}
