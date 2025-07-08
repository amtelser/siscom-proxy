#[derive(Debug)]
pub struct DeviceMessage {
    pub raw: String,
    pub device_id: String,
    pub header: String,
}


impl DeviceMessage {
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

    pub fn ack(&self) -> String {
        format!("{};{}", self.header, self.device_id)
    }
}

