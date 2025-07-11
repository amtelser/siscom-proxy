/// Represents a message received from a device.
/// 
/// - `raw`: The original message string.
/// - `device_id`: The identifier of the device.
/// - `header`: The message header, with certain keywords replaced for ACKs.
#[derive(Debug)]
pub struct DeviceMessage {
    pub raw: String,
    pub device_id: String,
    pub header: String,
}

impl DeviceMessage {
    /// Parses a raw message string into a `DeviceMessage`.
    /// 
    /// The message is expected to be separated by semicolons.
    /// The header is modified to replace "STT" or "ALT" with "ACK".
    /// Returns `None` if the message format is invalid.
    pub fn parse(raw: &str) -> Option<Self> {
        let parts: Vec<&str> = raw.trim().split(';').collect();
        if parts.len() < 2 {
            return None;
        }
        let header = parts[0].replace("STT", "ACK").replace("ALT", "ACK");
        Some(Self {
            raw: raw.to_string(),
            device_id: parts[1].to_string(),
            header,
        })
    }

    /// Generates an ACK message string for this device message.
    /// 
    /// The ACK consists of the header and device ID separated by a semicolon.
    pub fn ack(&self) -> String {
        format!("{};{}", self.header, self.device_id)
    }
}

