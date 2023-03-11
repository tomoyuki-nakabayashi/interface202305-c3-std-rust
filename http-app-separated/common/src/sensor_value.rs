use serde::{Deserialize, Serialize};

/// Sensor value used between HTTP client and server communication.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum SensorValue {
    Temperature(f32),
    Humidity(f32),
    Pressure(f32),
}

impl ToString for SensorValue {
    fn to_string(&self) -> String {
        match *self {
            Self::Temperature(t) => {
                format!("temp: {:.2}", t)
            }
            Self::Humidity(h) => {
                format!("humi: {:.2}", h)
            }
            Self::Pressure(p) => {
                format!("pres: {:.2}", p)
            }
        }
    }
}
