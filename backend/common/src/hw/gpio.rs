use gpio_cdev::{Chip, LineRequestFlags};
use tracing::debug;

const GPIO_PC8_ORANGEPI_ZERO_3 : u32 = 72;
const GPIO_PC9_ORANGEPI_ZERO_3 : u32 = 73;

/// Placeholder for opening the lock
pub fn gpio_indicate_user_authorized() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GPIO chip
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let offset = GPIO_PC9_ORANGEPI_ZERO_3;  // PC9
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

    let offset = GPIO_PC8_ORANGEPI_ZERO_3; // PC6
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

