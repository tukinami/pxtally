#[derive(Debug)]
pub(crate) enum PxTallyError {
    #[allow(unused)]
    ImageError(image::ImageError),
    #[allow(unused)]
    Io(std::io::Error),
    #[allow(unused)]
    Serialize(serde_json::Error),
    #[allow(unused)]
    SystemTime(std::time::SystemTimeError),
}

impl From<image::ImageError> for PxTallyError {
    fn from(value: image::ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<std::io::Error> for PxTallyError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for PxTallyError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialize(value)
    }
}

impl From<std::time::SystemTimeError> for PxTallyError {
    fn from(value: std::time::SystemTimeError) -> Self {
        Self::SystemTime(value)
    }
}
