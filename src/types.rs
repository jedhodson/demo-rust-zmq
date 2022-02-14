#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TemperatureHumiditySensor {
    pub address: u8,
    pub temperature: u32,
    pub humidity: f32
}

pub const PIPE_NAME: &str = "ipc:///home/jedhodson/tmp/pipe";