use std::path::Path;

use image::{ImageReader, RgbImage};

use crate::config::{Cli, Commands};

mod hsl;
mod img_oklch;
mod oklch;

#[derive(Debug)]
pub(crate) enum ProcessError {
    #[allow(unused)]
    ImageError(image::ImageError),
    #[allow(unused)]
    Io(std::io::Error),
}

impl From<image::ImageError> for ProcessError {
    fn from(value: image::ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<std::io::Error> for ProcessError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub(crate) fn process(cli: &Cli) {
    if let Err(e) = process_body(cli) {
        eprintln!("{:?}", e);
    }
}

fn process_body(cli: &Cli) -> Result<(), ProcessError> {
    match &cli.command {
        Commands::Hsl(command) => hsl::process_hsl(command),
        Commands::Oklch(command) => oklch::process_oklch(command),
        Commands::ImgOklch(args) => img_oklch::process_img_oklch(args),
    }
}

pub(crate) fn load_image<P>(path: P) -> Result<RgbImage, ProcessError>
where
    P: AsRef<Path>,
{
    let image_raw = ImageReader::open(path.as_ref())?
        .with_guessed_format()?
        .decode()?;
    Ok(image_raw.to_rgb8())
}
