use std::path::Path;

use image::{ImageReader, RgbImage};

use crate::{
    config::{Cli, Commands},
    error::PxTallyError,
};

pub(crate) mod hsl;
mod img_oklch;
pub(crate) mod oklch;

pub(crate) fn process(cli: &Cli) {
    if let Err(e) = process_body(cli) {
        eprintln!("{:?}", e);
    }
}

fn process_body(cli: &Cli) -> Result<(), PxTallyError> {
    match &cli.command {
        Commands::Hsl(command) => hsl::process_hsl(command),
        Commands::Oklch(command) => oklch::process_oklch(command),
        Commands::ImgOklch(args) => img_oklch::process_img_oklch(args),
    }
}

pub(crate) fn load_image<P>(path: P) -> Result<RgbImage, PxTallyError>
where
    P: AsRef<Path>,
{
    let image_raw = ImageReader::open(path.as_ref())?
        .with_guessed_format()?
        .decode()?;
    Ok(image_raw.to_rgb8())
}
