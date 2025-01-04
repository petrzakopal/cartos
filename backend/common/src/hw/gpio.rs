use gpio_cdev::{Chip, LineRequestFlags};
use tracing::{debug, error};

const GPIO_PC8_ORANGEPI_ZERO_3: u32 = 72;
const GPIO_PC9_ORANGEPI_ZERO_3: u32 = 73;
const GPIO_PC6_ORANGEPI_ZERO_3 : u32 = 70;

/// Placeholder for opening the lock
pub fn gpio_indicate_user_authorized() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GPIO chip
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let offset = GPIO_PC9_ORANGEPI_ZERO_3; // PC9
    let handle = chip
        .get_line(offset)?
        .request(LineRequestFlags::OUTPUT, 0, "rust-gpio")?;

    // Set the output to high
    handle.set_value(1)?;
    debug!("Set PC9 to HIGH");

    // Wait for a bit (optional)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Set the output to low
    handle.set_value(0)?;
    debug!("Set PC9 to LOW");

    Ok(())
}

pub fn gpio_indicate_no_readers_found() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GPIO chip
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let offset = GPIO_PC8_ORANGEPI_ZERO_3; // PC8
    let handle = chip
        .get_line(offset)?
        .request(LineRequestFlags::OUTPUT, 0, "rust-gpio")?;

    // Set the output to high
    handle.set_value(1)?;
    debug!("Set PC6 to HIGH");

    // Wait for a bit (optional)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Set the output to low
    handle.set_value(0)?;
    debug!("Set PC6 to LOW");

    Ok(())
}

pub fn gpio_set_all_to_low() {

    gpio_safely_set_out_val("/dev/gpiochip0", GPIO_PC8_ORANGEPI_ZERO_3, "PC8", 0);
    gpio_safely_set_out_val("/dev/gpiochip0", GPIO_PC9_ORANGEPI_ZERO_3, "PC9", 0);
    // Open the GPIO chip
//    let mut chip = Chip::new("/dev/gpiochip0");
//
//    match chip {
//        Ok(mut chip) => {
//            debug!("Successfully obtained the gpio chip.");
//
//            let handle = chip.get_line(GPIO_PC8_ORANGEPI_ZERO_3);
//
//            match handle {
//                Ok(handle) => {
//                    let gpio = handle.request(LineRequestFlags::OUTPUT, 0, "rust-gpio");
//                    match gpio {
//                        Ok(gpio) => {
//                            debug!("Successfully obtained the gpio.");
//
//                            debug!("Set PC6 to LOW");
//                            // Set the output to low
//                            let val = gpio.set_value(0);
//                            match val {
//                                Ok(v) => {
//                                    debug!("Successfully set value of PC6 to LOW.")
//                                }
//                                Err(e) => {
//                                    debug!("Could not set value of PC6 to LOW. {:#?}", e)
//                                }
//                            };
//                        }
//                        Err(e) => {
//                            debug!("Could not obtain the gpio. {:#?}", e)
//                        }
//                    };
//                }
//                Err(e) => {
//                    error!("Error getting hangle for PC8. {:#?}", e);
//                }
//            };
//        }
//        Err(e) => {
//            error!("Could not obtain the gpio chip, {:#?}", e);
//        }
//    };
}

pub fn gpio_safely_set_out_val(
    chip_name: &str,
    gpio_number: u32,
    gpio_human_readable_name: &str,
    level: u8,
) {
    // Open the GPIO chip
    let chip = Chip::new(chip_name);

    match chip {
        Ok(mut chip) => {
            debug!("Successfully obtained the gpio {} chip.", chip_name);

            let handle = chip.get_line(gpio_number);

            match handle {
                Ok(handle) => {
                    let gpio = handle.request(LineRequestFlags::OUTPUT, level, "rust-gpio");
                    debug!("Successfully obtained the gpio handle.");
                    match gpio {
                        Ok(gpio) => {
                            debug!(
                                "Successfully obtained the gpio {}, human readable: {}.",
                                gpio_number, gpio_human_readable_name
                            );

                            debug!("Set {} to {}", gpio_human_readable_name, level);
                            // Set the output to low
                            let val = gpio.set_value(0);
                            match val {
                                Ok(v) => {
                                    debug!(
                                        "Successfully set value of {} to {}.",
                                        gpio_human_readable_name, level
                                    )
                                }
                                Err(e) => {
                                    debug!(
                                        "Could not set value of {} to {}. {:#?}",
                                        gpio_human_readable_name, level, e
                                    )
                                }
                            };
                        }
                        Err(e) => {
                            debug!(
                                "Could not obtain the gpio {}. {:#?}",
                                gpio_human_readable_name, e
                            )
                        }
                    };
                }
                Err(e) => {
                    error!(
                        "Error getting hangle for {}. {:#?}",
                        gpio_human_readable_name, e
                    );
                }
            };
        }
        Err(e) => {
            error!("Could not obtain the gpio chip: {}, {:#?}", chip_name, e);
        }
    };
}

pub async fn gpio_unplug_and_plug_nfc_usb() {

    gpio_safely_set_out_val("/dev/gpiochip0", GPIO_PC6_ORANGEPI_ZERO_3, "PC6", 0);
    // Must test which works better
    //tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

    std::thread::sleep(std::time::Duration::from_millis(1500));

    gpio_safely_set_out_val("/dev/gpiochip0", GPIO_PC6_ORANGEPI_ZERO_3, "PC6", 1);

}
