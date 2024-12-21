use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct CameraInfo {
    pub camera: Option<String>,
    pub exposure: Option<String>,
    pub exposure_bias: Option<String>,
    pub aperture: Option<String>,
    pub iso: Option<u16>,
    pub focal: Option<f64>,
    pub flash: Option<String>,
}

impl Display for CameraInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}, Exposure: {}, Bias: {}, Aperture: {}, ISO: {}, Focal: {}, Flash: {}",
            self.camera
                .as_ref()
                .unwrap_or(&"Unknown camera".to_string()),
            self.exposure.as_ref().unwrap_or(&"Undefined".to_string()),
            self.exposure_bias
                .as_ref()
                .unwrap_or(&"Undefined".to_string()),
            self.aperture.as_ref().unwrap_or(&"Undefined".to_string()),
            match self.iso {
                Some(v) => v.to_string(),
                None => "Undefined".to_string(),
            },
            match self.focal {
                Some(v) => format!("{} mm", v),
                None => "Undefined".to_string(),
            },
            self.flash.as_ref().unwrap_or(&"Undefined".to_string()),
        )
    }
}
