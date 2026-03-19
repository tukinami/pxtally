use color::{Oklab, OpaqueColor};
use image::{Rgb, RgbImage};

use crate::{
    config::{OklabCommands, PercentageArgs},
    counter::{count_by_func_with_filter, create_counters, Angle, Filter, PercentageCounter},
    error::PxTallyError,
    output::output,
    process::load_image,
};

pub(crate) mod constants {
    pub(crate) const LIGHTNESS_MIN: f32 = 0.0;
    pub(crate) const LIGHTNESS_MAX: f32 = 1.0;
    pub(crate) const A_MIN: f32 = -0.5;
    pub(crate) const A_MAX: f32 = 0.5;
    pub(crate) const A_WIDTH: f32 = 1.0;
    pub(crate) const B_MIN: f32 = -0.5;
    pub(crate) const B_MAX: f32 = 0.5;
    pub(crate) const B_WIDTH: f32 = 1.0;
}

struct OklabFilter {}

impl OklabFilter {
    pub fn new() -> OklabFilter {
        OklabFilter {}
    }
}

impl Filter<OpaqueColor<Oklab>> for OklabFilter {
    fn contains(&self, _target: &OpaqueColor<Oklab>) -> bool {
        true
    }

    fn to_target(pixel: &Rgb<u8>) -> OpaqueColor<Oklab> {
        let color_rgb = OpaqueColor::from_rgb8(pixel.0[0], pixel.0[1], pixel.0[2]);
        color_rgb.convert::<Oklab>()
    }

    fn hue_filter(&self) -> Option<&Angle> {
        None
    }
}

pub(crate) fn process_oklab(command: &OklabCommands) -> Result<(), PxTallyError> {
    match &command {
        OklabCommands::Lightness(args) => {
            let rgb_image = load_image(&args.path)?;
            process_lightness(&rgb_image, args)?;
        }
        OklabCommands::A(args) => {
            let rgb_image = load_image(&args.path)?;
            process_a(&rgb_image, args)?;
        }
        OklabCommands::B(args) => {
            let rgb_image = load_image(&args.path)?;
            process_b(&rgb_image, args)?;
        }
    }
    Ok(())
}

fn process_lightness(rgb_image: &RgbImage, args: &PercentageArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::LIGHTNESS_MIN,
        constants::LIGHTNESS_MAX,
        PercentageCounter::new,
    );

    let filter = OklabFilter::new();

    let extracted_totals =
        count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_lightness);

    output(
        "oklab",
        "lightness",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn process_a(rgb_image: &RgbImage, args: &PercentageArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::A_MIN,
        constants::A_WIDTH,
        PercentageCounter::new,
    );

    let filter = OklabFilter::new();

    let extracted_totals = count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_a);

    output(
        "oklab",
        "a",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn process_b(rgb_image: &RgbImage, args: &PercentageArgs) -> Result<(), PxTallyError> {
    let mut counters = create_counters(
        args.divisor,
        constants::B_MIN,
        constants::B_WIDTH,
        PercentageCounter::new,
    );

    let filter = OklabFilter::new();

    let extracted_totals = count_by_func_with_filter(rgb_image, &mut counters, &filter, pixel_to_b);

    output(
        "oklab",
        "b",
        &counters,
        rgb_image,
        &filter,
        &args.output,
        extracted_totals,
    )?;

    Ok(())
}

fn pixel_to_lightness(oklab: &OpaqueColor<Oklab>) -> f32 {
    oklab.components[0]
}

fn pixel_to_a(oklab: &OpaqueColor<Oklab>) -> f32 {
    oklab.components[1]
}

fn pixel_to_b(oklab: &OpaqueColor<Oklab>) -> f32 {
    oklab.components[2]
}

#[cfg(test)]
mod tests {
    use color::{Oklab, OpaqueColor};

    #[test]
    fn checking_value() {
        let target = OpaqueColor::from_rgb8(255, 255, 255);
        let oklab = target.convert::<Oklab>();
        println!("{}", oklab.components[0]);
        assert_eq!(oklab.components[0], 1.0);

        let target = OpaqueColor::from_rgb8(0, 0, 0);
        let oklab = target.convert::<Oklab>();
        println!("{}", oklab.components[0]);
        assert_eq!(oklab.components[1], 0.0);
        assert_eq!(oklab.components[0], 0.0);
    }
}
