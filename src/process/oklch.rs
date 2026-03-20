use color::{Oklch, OpaqueColor};
use image::{Rgb, RgbImage};

use crate::{
    config::{AngleArgs, OklchCommands, ValueWithHArgs},
    counter::{
        count_by_func_with_filter, create_counters, Angle, AngleCounter, Filter, PercentageCounter,
    },
    error::PxTallyError,
    output::output,
    process::load_image,
};

pub(crate) mod constants {
    pub(crate) const LIGHTNESS_MIN: f32 = 0.0;
    pub(crate) const LIGHTNESS_MAX: f32 = 1.0;
    pub(crate) const CHROMA_MIN: f32 = 0.0;
    pub(crate) const CHROMA_MAX: f32 = 0.5;
    pub(crate) const HUE_MIN: f32 = 0.0;
    pub(crate) const HUE_MAX: f32 = 360.0;
}

struct OklchFilter {
    hue_filter: Option<Angle>,
}

impl OklchFilter {
    pub fn new(start_hue: Option<&u16>, end_hue: Option<&u16>) -> OklchFilter {
        let hue_filter =
            <OklchFilter as Filter<OpaqueColor<Oklch>>>::create_hue_filter(start_hue, end_hue);

        OklchFilter { hue_filter }
    }
}

impl Filter<OpaqueColor<Oklch>> for OklchFilter {
    fn contains(&self, target: &OpaqueColor<Oklch>) -> bool {
        self.hue_filter
            .as_ref()
            .map(|v| v.contains(&target.components[2]))
            .unwrap_or(true)
    }

    fn to_target(pixel: &Rgb<u8>) -> OpaqueColor<Oklch> {
        let color_rgb = OpaqueColor::from_rgb8(pixel.0[0], pixel.0[1], pixel.0[2]);
        color_rgb.convert::<Oklch>()
    }

    fn hue_filter(&self) -> Option<&Angle> {
        self.hue_filter.as_ref()
    }
}

pub(crate) fn process_oklch(command: &OklchCommands) -> Result<(), PxTallyError> {
    match &command {
        OklchCommands::Lightness(args) => {
            let rgb_image = load_image(&args.path)?;
            process_lightness(&rgb_image, args)?;
        }
        OklchCommands::Chroma(args) => {
            let rgb_image = load_image(&args.path)?;
            process_chroma(&rgb_image, args)?;
        }
        OklchCommands::Hue(args) => {
            let rgb_image = load_image(&args.path)?;
            process_hue(&rgb_image, args)?;
        }
    }
    Ok(())
}

fn process_lightness(rgb_image: &RgbImage, args: &ValueWithHArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::LIGHTNESS_MIN,
        constants::LIGHTNESS_MAX,
        PercentageCounter::new,
    );

    let filter = OklchFilter::new(args.start_hue.as_ref(), args.end_hue.as_ref());

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_lightness);

    output(
        "oklch",
        "lightness",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn process_chroma(rgb_image: &RgbImage, args: &ValueWithHArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::CHROMA_MIN,
        constants::CHROMA_MAX,
        PercentageCounter::new,
    );

    let filter = OklchFilter::new(args.start_hue.as_ref(), args.end_hue.as_ref());

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_chroma);

    output(
        "oklch",
        "chroma",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn process_hue(rgb_image: &RgbImage, args: &AngleArgs) -> Result<(), PxTallyError> {
    let start = (args.start % 360) as f32;
    let mut counters = create_counters(args.divisor, start, constants::HUE_MAX, AngleCounter::new);

    let filter = OklchFilter::new(None, None);

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_hue);

    output(
        "oklch",
        "hue",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn pixel_to_lightness(oklch: &OpaqueColor<Oklch>) -> f32 {
    oklch.components[0]
}

fn pixel_to_chroma(oklch: &OpaqueColor<Oklch>) -> f32 {
    oklch.components[1]
}

fn pixel_to_hue(oklch: &OpaqueColor<Oklch>) -> f32 {
    oklch.components[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    mod oklch_filter {
        use super::*;

        mod contains {
            use image::Pixel;

            use super::*;

            #[test]
            fn checking_value() {
                let filter = OklchFilter::new(Some(&90), Some(&200));
                let pixel = Rgb::from_slice(&[0_u8, 255_u8, 0_u8]);
                let case = OklchFilter::to_target(pixel);
                assert!(filter.contains(&case));

                let filter = OklchFilter::new(Some(&90), Some(&200));
                let pixel = Rgb::from_slice(&[255_u8, 0_u8, 0_u8]);
                let case = OklchFilter::to_target(pixel);
                assert!(!filter.contains(&case));

                let filter = OklchFilter::new(None, None);
                let pixel = Rgb::from_slice(&[255_u8, 0_u8, 0_u8]);
                let case = OklchFilter::to_target(pixel);
                assert!(filter.contains(&case));
            }
        }

        mod to_target {
            use image::Pixel;

            use super::*;

            #[test]
            fn checking_value() {
                let case = image::Rgb::from_slice(&[255_u8, 255_u8, 255_u8]);
                let result = OklchFilter::to_target(case);
                assert_eq!(result.components[0], constants::LIGHTNESS_MAX);

                let case = image::Rgb::from_slice(&[0_u8, 0_u8, 0_u8]);
                let result = OklchFilter::to_target(case);
                assert_eq!(result.components[0], constants::LIGHTNESS_MIN);

                let case = image::Rgb::from_slice(&[0_u8, 255_u8, 0_u8]);
                let result = OklchFilter::to_target(case);
                assert_ne!(result.components[1], constants::CHROMA_MIN);
                assert!((60.0..180.0).contains(&result.components[2]));

                let case = image::Rgb::from_slice(&[0_u8, 0_u8, 255_u8]);
                let result = OklchFilter::to_target(case);
                assert_ne!(result.components[1], constants::CHROMA_MIN);
                assert!(!(0.0..160.0).contains(&result.components[2]));
            }
        }
    }

    mod pixel_to {
        use super::*;

        use color::{Oklch, OpaqueColor};

        #[test]
        fn checking_value() {
            let target = OpaqueColor::from_rgb8(255, 255, 255);
            let oklch = target.convert::<Oklch>();
            assert_eq!(pixel_to_lightness(&oklch), constants::LIGHTNESS_MAX);

            let target = OpaqueColor::from_rgb8(0, 0, 0);
            let oklch = target.convert::<Oklch>();
            assert_eq!(pixel_to_lightness(&oklch), constants::LIGHTNESS_MIN);
            assert_eq!(pixel_to_chroma(&oklch), constants::CHROMA_MIN);

            let target = OpaqueColor::from_rgb8(0, 255, 0);
            let oklch = target.convert::<Oklch>();
            assert!((60.0..180.0).contains(&pixel_to_hue(&oklch)));
        }
    }
}
