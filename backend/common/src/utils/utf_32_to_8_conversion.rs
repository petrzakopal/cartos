pub struct Utf32To8<'a, T>(pub &'a Vec<T>);
// ------------- u8 ------------- //
impl<'a> Utf32To8<'a, u8> {
    pub fn to_hex_raw(&self) -> Vec<u8> {
        let mut raw_bytes = Vec::new();
        for value in self.0 {
            let bytes = value.to_be_bytes();
            raw_bytes.extend_from_slice(&bytes);
        }
        raw_bytes
    }

    // Method to display the raw bytes in hexadecimal format in string
    pub fn display_raw_string(&self) -> String {
        self.to_hex_raw()
            .iter()
            .map(|&byte| format!("0x{:02X}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn serial_number_format(&self) -> String {
        self.to_hex_raw()
            .iter()
            .map(|&byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(":")
    }
}
