/// Available card types from which read the data
pub enum CardType {
    Mifare,
    Ntag215,
    Unknown
}

/// Card data or properties which can be read using the reader
pub enum CardDataSelect {
    SerialNumber,
}
