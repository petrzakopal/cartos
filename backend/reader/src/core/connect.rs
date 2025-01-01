use std::ffi::CStr;

use common::types::{
    card::{CardDataSelect, CardType},
    channels::CardData,
};
use pcsc::*;
use tracing::info;
use tracing_log::log::debug;

use crate::core::commands::read_serial_number_module::read_serial_number;



/// Checks if the reader is dead
fn is_dead(rs: &ReaderState) -> bool {
    rs.event_state().intersects(State::UNKNOWN | State::IGNORE)
}

// Removes dead readers and preserves the live readers
fn remove_dead_reader(mut reader_states: Vec<ReaderState>) -> Vec<ReaderState> {
    for rs in &reader_states {
        if is_dead(rs) {
            info!("Removing dead reader: {:?}", rs.name());
        }
    }

    // Actually removing the dead readers
    // when !is_reader is false - it is removed from the vec
    reader_states.retain(|rs| !is_dead(rs));

    return reader_states;
}

fn add_new_reader<'a>(
    ctx: &Context,
    mut reader_states: Vec<ReaderState>,
    readers_buf: &'a mut [u8; 2048],
) -> (Vec<ReaderState>, Vec<&'a CStr>) {
    //let mut readers_buf_clone = readers_buf.clone();

    // is is in bytes, do not know if needed, but will see
    let reader_names = ctx
        .list_readers(readers_buf)
        .expect("Failed to get the list of readers.");

    let mut readers = ctx
        .list_readers(readers_buf)
        .expect("Failed to get the list of readers.")
        .collect::<Vec<_>>();

    info!("Obtained readers {:?}", readers);
    //info!("Obtained reader names {:?}", reader_names);

    for name in readers.clone() {
        if !reader_states.iter().any(|rs| rs.name() == name) {
            info!("Adding reader {:?}", name);
            reader_states.push(ReaderState::new(name, State::UNAWARE));
        }
    }

    for rs in &mut reader_states {
        rs.sync_current_state();
    }
    return (reader_states, readers);
}

fn print_reader_current_state(reader_states: &Vec<ReaderState>) {
    for rs in reader_states {
        if rs.name() != PNP_NOTIFICATION() {
            debug!(
                "Reader current state: {:?} {:?} {:?}",
                rs.name(),
                rs.event_state(),
                rs.atr()
            );
        }
    }
}

fn check_reader_current_state(reader_states: &Vec<ReaderState>, current_state: State) -> bool {
    for rs in reader_states {
        if rs.name() != PNP_NOTIFICATION() {
            let name = rs.name().to_string_lossy();

            if name.contains("ACR122U") {
                //debug!("Contains ACR");
                if rs.current_state().contains(current_state) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn check_reader_event_state(reader_states: &Vec<ReaderState>, event_state: State) -> bool {
    // Loop through every reader
    for rs in reader_states {
        // Check if there is no notification status so that everything is fine
        if rs.name() != PNP_NOTIFICATION() {
            // Convert current checked reader name to String
            let name = rs.name().to_string_lossy();

            // Check if it is a reader the software can use
            if name.contains("ACR122U") {
                // Check for the event state
                if rs.event_state().contains(event_state) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn check_reader_current_and_event_state(
    reader_states: &Vec<ReaderState>,
    current_state: State,
    event_state: State,
) -> (bool, bool) {
    let mut current_state_bool: bool = false;
    let mut event_state_bool: bool = false;

    // Loop through every reader
    for rs in reader_states {
        // Check if there is no notification status so that everything is fine
        if rs.name() != PNP_NOTIFICATION() {
            // Convert current checked reader name to String
            let name = rs.name().to_string_lossy();

            // Check if it is a reader the software can use
            if name.contains("ACR122U") {
                // Check for the event state
                if rs.event_state().contains(event_state) {
                    event_state_bool = true;
                }

                if rs.current_state().contains(current_state) {
                    current_state_bool = true;
                }
            }
        }
    }

    return (current_state_bool, event_state_bool);
}

pub fn initialize_readers() -> (Context, Vec<pcsc::ReaderState>, [u8; 2048]) {
    // Create context
    let ctx =
        Context::establish(Scope::User).expect("Failed to estabilish context for the reader.");

    // Do not know if used, it is used for storing
    // the raw bytes of the reader names
    let mut readers_buf: [u8; 2048] = [0; 2048];
    // Used for reader states
    let mut reader_states = vec![ReaderState::new(PNP_NOTIFICATION(), State::UNAWARE)];

    reader_states = remove_dead_reader(reader_states);

    let mut readers: Vec<&CStr> = Vec::new();

    (reader_states, readers) = add_new_reader(&ctx, reader_states, &mut readers_buf);

    print_reader_current_state(&reader_states);
    // wait until state changes to be able to communicate
    ctx.get_status_change(None, &mut reader_states)
        .expect("Failed to get the status change.");
    print_reader_current_state(&reader_states);

    return (ctx, reader_states, readers_buf);
}

pub async fn read_loop(serial_number_channel_sender: tokio::sync::broadcast::Sender<CardData>) {
    // Initialize readers
    debug!("Initialize the readers.");
    let (mut ctx, mut reader_states, mut readers_buf): (
        Context,
        Vec<pcsc::ReaderState>,
        [u8; 2048],
    ) = initialize_readers();

    let mut card_processed = false;
    let mut card_data : CardData = CardData::new();
    //let mut card_data: CardData = CardData {
    //    serial_number_string: String::default(),
    //};

    let mut card_data_to_read: CardDataSelect = CardDataSelect::SerialNumber;

    loop {
        match ctx.get_status_change(Some(std::time::Duration::from_secs(5)), &mut reader_states) {
            Ok(_) => {
                if !check_reader_current_state(&reader_states, State::PRESENT)
                    && check_reader_event_state(&reader_states, State::PRESENT)
                {
                    // Processing the inserted card for the first time
                    if !card_processed {
                        info!("Card detected in the reader");

                        // data reading selection and machine
                        match card_data_to_read {
                            CardDataSelect::SerialNumber => {
                               let serial_number_string = read_serial_number(&ctx, readers_buf, CardType::Unknown);
                                card_data.serial_number_string = serial_number_string;
                               let _ = serial_number_channel_sender.send(card_data.clone());
                            }
                        };

                        // Process the card only once
                        card_processed = true;
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                    // The card has been removed from the reader
                } else if check_reader_event_state(&reader_states, State::EMPTY)
                    && !check_reader_current_state(&reader_states, State::EMPTY)
                {
                    card_processed = false;
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            }
            Err(e) => {
                // Handle timeout or error
                if e == pcsc::Error::Timeout {
                    // Normal timeout, add a small delay
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                } else {
                    info!("Error getting status change: {}", e);
                    // Add longer delay on error
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}
