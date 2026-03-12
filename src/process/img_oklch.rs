use std::{collections::HashMap, path::PathBuf};

use crate::{
    config::ImgOklchArgs, error::PxTallyError, output::confirm_and_run, process::load_image,
};

use color::{Oklch, OpaqueColor, Rgba8};
use image::RgbImage;

pub(crate) fn process_img_oklch(args: &ImgOklchArgs) -> Result<(), PxTallyError> {
    let mut rgb_image = load_image(&args.input)?;
    let pixels = rgb_image.pixels_mut();
    let mut record = HashMap::new();

    for pixel in pixels {
        let color_rgb =
            oklch_to_adjust_rgb(&mut record, &pixel.0, args.lightness, args.chroma, args.hue);
        pixel.0[0] = color_rgb.r;
        pixel.0[1] = color_rgb.g;
        pixel.0[2] = color_rgb.b;
    }

    if args.output.exists() && !args.force {
        confirm_and_run(&args.output, || write_rgb_image(&args.output, &rgb_image))?;
    } else {
        write_rgb_image(&args.output, &rgb_image)?;
    }

    Ok(())
}

fn write_rgb_image(path: &PathBuf, rgb_image: &RgbImage) -> Result<(), PxTallyError> {
    rgb_image.save(path)?;
    Ok(())
}

fn oklch_to_adjust_rgb(
    record: &mut HashMap<[u8; 3], Rgba8>,
    pixel: &[u8; 3],
    lightness: Option<f32>,
    chroma: Option<f32>,
    hue: Option<u16>,
) -> Rgba8 {
    if let Some(v) = record.get(pixel) {
        return *v;
    }

    let color_rgb = OpaqueColor::from_rgb8(pixel[0], pixel[1], pixel[2]);
    let mut oklch = color_rgb.convert::<Oklch>();

    if let Some(l) = lightness {
        oklch.components[0] = l;
    }
    if let Some(c) = chroma {
        oklch.components[1] = c;
    }
    if let Some(h) = hue {
        oklch.components[2] = h as f32;
    }
    let rgba = oklch.to_rgba8();
    record.insert(*pixel, rgba);

    rgba
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    mod process_img_oklch {
        use super::*;

        #[test]
        fn check_file() {
            let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target");
            let base_file = base_dir.join("base.png");

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l050_c015.png",
                None,
                None,
                None,
            );
            assert!(process_img_oklch(&args).is_ok());

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l050_c015.png",
                Some(0.5),
                Some(0.15),
                None,
            );
            assert!(process_img_oklch(&args).is_ok());

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l050_c010.png",
                Some(0.5),
                Some(0.10),
                None,
            );
            assert!(process_img_oklch(&args).is_ok());

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l050_c005.png",
                Some(0.5),
                Some(0.05),
                None,
            );
            assert!(process_img_oklch(&args).is_ok());

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l075_c010.png",
                Some(0.75),
                Some(0.10),
                None,
            );
            assert!(process_img_oklch(&args).is_ok());

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l025_c010.png",
                Some(0.25),
                Some(0.10),
                None,
            );
            assert!(process_img_oklch(&args).is_ok());

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l050_h300.png",
                Some(0.5),
                None,
                Some(300),
            );
            assert!(process_img_oklch(&args).is_ok());

            let args = create_args(
                &base_dir,
                &base_file,
                "test_l050_c012.png",
                Some(0.5),
                Some(0.12),
                None,
            );
            assert!(process_img_oklch(&args).is_ok());
        }

        fn create_args(
            base_dir: &PathBuf,
            input: &PathBuf,
            output: &str,
            lightness: Option<f32>,
            chroma: Option<f32>,
            hue: Option<u16>,
        ) -> ImgOklchArgs {
            let input = input.clone();
            let output = base_dir.join(output);
            ImgOklchArgs {
                input,
                output,
                lightness,
                chroma,
                hue,
                force: true,
            }
        }
    }
}
