use color::{Lab, OpaqueColor};
use image::{Rgb, RgbImage};

use crate::{
    config::{CielabCommands, ValueArgs},
    counter::{count_by_func_with_filter, create_counters, Angle, Filter, PercentageCounter},
    error::PxTallyError,
    output::output,
    process::load_image,
};

pub(crate) mod constants {
    pub(crate) const LIGHTNESS_MIN: f32 = 0.0;
    pub(crate) const LIGHTNESS_MAX: f32 = 100.0;
    pub(crate) const A_MIN: f32 = -160.0;
    pub(crate) const A_MAX: f32 = 160.0;
    pub(crate) const A_WIDTH: f32 = 320.0;
    pub(crate) const B_MIN: f32 = -160.0;
    pub(crate) const B_MAX: f32 = 160.0;
    pub(crate) const B_WIDTH: f32 = 320.0;
}

struct CielabFilter {}

impl CielabFilter {
    pub fn new() -> CielabFilter {
        CielabFilter {}
    }
}

impl Filter<OpaqueColor<Lab>> for CielabFilter {
    fn contains(&self, _target: &OpaqueColor<Lab>) -> bool {
        true
    }

    fn to_target(pixel: &Rgb<u8>) -> OpaqueColor<Lab> {
        let color_rgb = OpaqueColor::from_rgb8(pixel.0[0], pixel.0[1], pixel.0[2]);
        color_rgb.convert::<Lab>()
    }

    fn hue_filter(&self) -> Option<&Angle> {
        None
    }
}

pub(crate) fn process_lab(command: &CielabCommands) -> Result<(), PxTallyError> {
    match &command {
        CielabCommands::Lightness(args) => {
            let rgb_image = load_image(&args.path)?;
            process_lightness(&rgb_image, args)?;
        }
        CielabCommands::A(args) => {
            let rgb_image = load_image(&args.path)?;
            process_a(&rgb_image, args)?;
        }
        CielabCommands::B(args) => {
            let rgb_image = load_image(&args.path)?;
            process_b(&rgb_image, args)?;
        }
    }
    Ok(())
}

fn process_lightness(rgb_image: &RgbImage, args: &ValueArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::LIGHTNESS_MIN,
        constants::LIGHTNESS_MAX,
        PercentageCounter::new,
    );

    let filter = CielabFilter::new();

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_lightness);

    output(
        "cielab",
        "lightness",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn process_a(rgb_image: &RgbImage, args: &ValueArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::A_MIN,
        constants::A_WIDTH,
        PercentageCounter::new,
    );

    let filter = CielabFilter::new();

    let extracted_totals = count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_a);

    output(
        "cielab",
        "a",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn process_b(rgb_image: &RgbImage, args: &ValueArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::B_MIN,
        constants::B_WIDTH,
        PercentageCounter::new,
    );

    let filter = CielabFilter::new();

    let extracted_totals = count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_b);

    output(
        "cielab",
        "b",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn pixel_to_lightness(lab: &OpaqueColor<Lab>) -> f32 {
    lab.components[0]
}

fn pixel_to_a(lab: &OpaqueColor<Lab>) -> f32 {
    lab.components[1]
}

fn pixel_to_b(lab: &OpaqueColor<Lab>) -> f32 {
    lab.components[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cielab_filter {
        use super::*;

        mod to_target {
            use image::Pixel;

            use super::*;

            #[test]
            fn checking_value() {
                let case = image::Rgb::from_slice(&[255_u8, 255_u8, 255_u8]);
                let result = CielabFilter::to_target(case);
                assert_eq!(result.components[0], constants::LIGHTNESS_MAX);

                let case = image::Rgb::from_slice(&[0_u8, 0_u8, 0_u8]);
                let result = CielabFilter::to_target(case);
                assert_eq!(result.components[0], constants::LIGHTNESS_MIN);

                let case = image::Rgb::from_slice(&[255_u8, 0_u8, 0_u8]);
                let result = CielabFilter::to_target(case);
                assert!(result.components[1] > 0.0);

                let case = image::Rgb::from_slice(&[0_u8, 255_u8, 0_u8]);
                let result = CielabFilter::to_target(case);
                assert!(result.components[1] < 0.0);

                let case = image::Rgb::from_slice(&[0_u8, 0_u8, 255_u8]);
                let result = CielabFilter::to_target(case);
                assert!(result.components[2] < 0.0);
            }
        }
    }

    mod pixel_to {
        use super::*;

        use color::{Lab, OpaqueColor};

        #[test]
        fn checking_value() {
            let target = OpaqueColor::from_rgb8(255, 255, 255);
            let lab = target.convert::<Lab>();
            assert_eq!(pixel_to_lightness(&lab), constants::LIGHTNESS_MAX);

            let target = OpaqueColor::from_rgb8(0, 0, 0);
            let lab = target.convert::<Lab>();
            assert_eq!(pixel_to_lightness(&lab), constants::LIGHTNESS_MIN);
            assert_eq!(pixel_to_a(&lab), 0.0);
            assert_eq!(pixel_to_b(&lab), 0.0);
        }
    }
}
