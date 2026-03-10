use std::collections::HashMap;

use crate::{
    config::ImgOklchArgs,
    process::{load_image, ProcessError},
};

use bigcolor::color_space::oklch_to_rgb;

pub(crate) fn process_img_oklch(args: &ImgOklchArgs) -> Result<(), ProcessError> {
    let mut rgb_image = load_image(&args.input)?;
    let pixels = rgb_image.pixels_mut();
    let mut record = HashMap::new();

    for pixel in pixels {
        let (r, g, b, _a) =
            oklch_to_adjust_rgb(&mut record, &pixel.0, args.lightness, args.chroma, args.hue);
        pixel.0[0] = r;
        pixel.0[1] = g;
        pixel.0[2] = b;
    }

    rgb_image.save(&args.output)?;
    Ok(())
}

fn oklch_to_adjust_rgb(
    record: &mut HashMap<[u8; 3], (u8, u8, u8, f32)>,
    pixel: &[u8; 3],
    lightness: Option<f32>,
    chroma: Option<f32>,
    hue: Option<u16>,
) -> (u8, u8, u8, f32) {
    if let Some(v) = record.get(pixel) {
        return *v;
    }

    let big_color = bigcolor::BigColor::from_rgb(pixel[0], pixel[1], pixel[2], 1.0);
    let mut oklch = big_color.to_oklch();
    if let Some(l) = lightness {
        oklch.l = l;
    }
    if let Some(c) = chroma {
        oklch.c = c;
    }
    if let Some(h) = hue {
        oklch.h = h as f32;
    }

    let rgba = oklch_to_rgb(oklch);
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
            }
        }
    }
}
