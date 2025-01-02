use gpio_cdev::{Chip, LineRequestFlags};
use tracing::debug;

pub fn gpio_set_1_then_0() -> Result<(), Box<dyn std::error::Error>> {
    // Open the GPIO chip
    let mut chip = Chip::new("/dev/gpiochip0")?;

    // Request the line for PC6 (replace '6' with the correct offset)
    let offset = 73; // Replace this with the actual offset for PC6
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
