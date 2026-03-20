use color::{Lch, OpaqueColor};
use image::{Rgb, RgbImage};

use crate::{
    config::{AngleArgs, CielchCommands, ValueWithHArgs},
    counter::{
        count_by_func_with_filter, create_counters, Angle, AngleCounter, Filter, PercentageCounter,
    },
    error::PxTallyError,
    output::output,
    process::load_image,
};

pub(crate) mod constants {
    pub(crate) const LIGHTNESS_MIN: f32 = 0.0;
    pub(crate) const LIGHTNESS_MAX: f32 = 100.0;
    pub(crate) const CHROMA_MIN: f32 = 0.0;
    pub(crate) const CHROMA_MAX: f32 = 160.0;
    #[allow(unused)]
    pub(crate) const HUE_MIN: f32 = 0.0;
    pub(crate) const HUE_MAX: f32 = 360.0;
}

struct CielchFilter {
    hue_filter: Option<Angle>,
}

impl CielchFilter {
    pub fn new(start_hue: Option<&u16>, end_hue: Option<&u16>) -> CielchFilter {
        let hue_filter =
            <CielchFilter as Filter<OpaqueColor<Lch>>>::create_hue_filter(start_hue, end_hue);

        CielchFilter { hue_filter }
    }
}

impl Filter<OpaqueColor<Lch>> for CielchFilter {
    fn contains(&self, target: &OpaqueColor<Lch>) -> bool {
        self.hue_filter
            .as_ref()
            .map(|v| v.contains(&target.components[2]))
            .unwrap_or(true)
    }

    fn to_target(pixel: &Rgb<u8>) -> OpaqueColor<Lch> {
        let color_rgb = OpaqueColor::from_rgb8(pixel.0[0], pixel.0[1], pixel.0[2]);
        color_rgb.convert::<Lch>()
    }

    fn hue_filter(&self) -> Option<&Angle> {
        self.hue_filter.as_ref()
    }
}

pub(crate) fn process_lch(command: &CielchCommands) -> Result<(), PxTallyError> {
    match &command {
        CielchCommands::Lightness(args) => {
            let rgb_image = load_image(&args.path)?;
            process_lightness(&rgb_image, args)?;
        }
        CielchCommands::Chroma(args) => {
            let rgb_image = load_image(&args.path)?;
            process_chroma(&rgb_image, args)?;
        }
        CielchCommands::Hue(args) => {
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

    let filter = CielchFilter::new(args.start_hue.as_ref(), args.end_hue.as_ref());

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_lightness);

    output(
        "cielch",
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

    let filter = CielchFilter::new(args.start_hue.as_ref(), args.end_hue.as_ref());

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_chroma);

    output(
        "cielch",
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

    let filter = CielchFilter::new(None, None);

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_hue);

    output(
        "cielch",
        "hue",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn pixel_to_lightness(lch: &OpaqueColor<Lch>) -> f32 {
    lch.components[0]
}

fn pixel_to_chroma(lch: &OpaqueColor<Lch>) -> f32 {
    lch.components[1]
}

fn pixel_to_hue(lch: &OpaqueColor<Lch>) -> f32 {
    lch.components[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cielch_filter {
        use super::*;

        mod contains {
            use image::Pixel;

            use super::*;

            #[test]
            fn checking_value() {
                let filter = CielchFilter::new(Some(&90), Some(&200));
                let pixel = Rgb::from_slice(&[0_u8, 255_u8, 0_u8]);
                let case = CielchFilter::to_target(pixel);
                assert!(filter.contains(&case));

                let filter = CielchFilter::new(Some(&90), Some(&200));
                let pixel = Rgb::from_slice(&[255_u8, 0_u8, 0_u8]);
                let case = CielchFilter::to_target(pixel);
                assert!(!filter.contains(&case));

                let filter = CielchFilter::new(None, None);
                let pixel = Rgb::from_slice(&[255_u8, 0_u8, 0_u8]);
                let case = CielchFilter::to_target(pixel);
                assert!(filter.contains(&case));
            }
        }

        mod to_target {
            use image::Pixel;

            use super::*;

            #[test]
            fn checking_value() {
                let case = image::Rgb::from_slice(&[255_u8, 255_u8, 255_u8]);
                let result = CielchFilter::to_target(case);
                assert_eq!(result.components[0], constants::LIGHTNESS_MAX);

                let case = image::Rgb::from_slice(&[0_u8, 0_u8, 0_u8]);
                let result = CielchFilter::to_target(case);
                assert_eq!(result.components[0], constants::LIGHTNESS_MIN);

                let case = image::Rgb::from_slice(&[0_u8, 255_u8, 0_u8]);
                let result = CielchFilter::to_target(case);
                assert_ne!(result.components[1], constants::CHROMA_MIN);
                assert!((60.0..180.0).contains(&result.components[2]));

                let case = image::Rgb::from_slice(&[0_u8, 0_u8, 255_u8]);
                let result = CielchFilter::to_target(case);
                assert_ne!(result.components[1], constants::CHROMA_MIN);
                assert!(!(0.0..160.0).contains(&result.components[2]));
            }
        }
    }

    mod pixel_to {
        use super::*;

        use color::{Lch, OpaqueColor};

        #[test]
        fn checking_value() {
            let target = OpaqueColor::from_rgb8(255, 255, 255);
            let lch = target.convert::<Lch>();
            assert_eq!(pixel_to_lightness(&lch), constants::LIGHTNESS_MAX);

            let target = OpaqueColor::from_rgb8(0, 0, 0);
            let lch = target.convert::<Lch>();
            assert_eq!(pixel_to_lightness(&lch), constants::LIGHTNESS_MIN);
            assert_eq!(pixel_to_chroma(&lch), constants::CHROMA_MIN);

            let target = OpaqueColor::from_rgb8(0, 255, 0);
            let lch = target.convert::<Lch>();
            assert!((60.0..180.0).contains(&pixel_to_hue(&lch)));
        }
    }
}
