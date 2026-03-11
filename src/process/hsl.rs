use color::{Hsl, OpaqueColor};
use image::{Rgb, RgbImage};

use crate::{
    config::{AngleArgs, HslCommands, PercentageArgs},
    counter::{
        count_by_func_with_filter, create_counters, print_count, Angle, AngleCounter, Filter,
        PercentageCounter,
    },
    process::{load_image, ProcessError},
};

pub(crate) mod constants {
    #[allow(unused)]
    pub(crate) const HUE_MIN: f32 = 0.0;
    pub(crate) const HUE_MAX: f32 = 360.0;
    pub(crate) const SATURATION_MIN: f32 = 0.0;
    pub(crate) const SATURATION_MAX: f32 = 100.0;
    pub(crate) const LIGHTNESS_MIN: f32 = 0.0;
    pub(crate) const LIGHTNESS_MAX: f32 = 100.0;
}

struct HslFilter {
    hue: Option<Angle>,
}

impl HslFilter {
    pub fn new(start_hue: &Option<u16>, end_hue: &Option<u16>) -> HslFilter {
        let hue = <HslFilter as Filter<OpaqueColor<Hsl>>>::hue_filter(start_hue, end_hue);

        HslFilter { hue }
    }
}

impl Filter<OpaqueColor<Hsl>> for HslFilter {
    fn contains(&self, target: &OpaqueColor<Hsl>) -> bool {
        self.hue
            .as_ref()
            .map(|v| v.contains(&target.components[0]))
            .unwrap_or(true)
    }

    fn to_target(pixel: &Rgb<u8>) -> OpaqueColor<Hsl> {
        let color_rgb = OpaqueColor::from_rgb8(pixel.0[0], pixel.0[1], pixel.0[2]);
        color_rgb.convert::<Hsl>()
    }
}

pub(crate) fn process_hsl(command: &HslCommands) -> Result<(), ProcessError> {
    match &command {
        HslCommands::Hue(args) => {
            let rgb_image = load_image(&args.path)?;
            process_hue(&rgb_image, args);
        }
        HslCommands::Saturation(args) => {
            let rgb_image = load_image(&args.path)?;
            process_saturation(&rgb_image, args);
        }
        HslCommands::Lightness(args) => {
            let rgb_image = load_image(&args.path)?;
            process_lightness(&rgb_image, args);
        }
    }
    Ok(())
}

fn process_hue(rgb_image: &RgbImage, args: &AngleArgs) {
    let start = (args.start % 360) as f32;
    let mut counters = create_counters(args.divisor, start, constants::HUE_MAX, AngleCounter::new);

    let filter = HslFilter::new(&None, &None);

    let (filtered_total_value, filtered_total_pixel) =
        count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_hue);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filtered_total_value,
        filtered_total_pixel,
    );
}

fn process_saturation(rgb_image: &RgbImage, args: &PercentageArgs) {
    let mut counters = create_counters(
        args.divisor,
        constants::SATURATION_MIN,
        constants::SATURATION_MAX,
        PercentageCounter::new,
    );

    let filter = HslFilter::new(&None, &None);

    let (filtered_total_value, filtered_total_pixel) =
        count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_saturation);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filtered_total_value,
        filtered_total_pixel,
    );
}

fn process_lightness(rgb_image: &RgbImage, args: &PercentageArgs) {
    let mut counters = create_counters(
        args.divisor,
        constants::LIGHTNESS_MIN,
        constants::LIGHTNESS_MAX,
        PercentageCounter::new,
    );

    let filter = HslFilter::new(&None, &None);

    let (filtered_total_value, filtered_total_pixel) =
        count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_lightness);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filtered_total_value,
        filtered_total_pixel,
    );
}

fn pixel_to_hue(hsl: &OpaqueColor<Hsl>) -> f32 {
    hsl.components[0]
}

fn pixel_to_saturation(hsl: &OpaqueColor<Hsl>) -> f32 {
    hsl.components[1]
}

fn pixel_to_lightness(hsl: &OpaqueColor<Hsl>) -> f32 {
    hsl.components[2]
}

#[cfg(test)]
mod tests {
    use color::{Hsl, OpaqueColor};

    #[test]
    fn checking_value() {
        let target = OpaqueColor::from_rgb8(255, 255, 255);
        let hsl = target.convert::<Hsl>();
        println!("{}", hsl.components[2]);
        assert_eq!(hsl.components[1], 0.0);
        assert_eq!(hsl.components[2], 100.0);

        let target = OpaqueColor::from_rgb8(0, 0, 0);
        let hsl = target.convert::<Hsl>();
        println!("{}", hsl.components[2]);
        assert_eq!(hsl.components[2], 0.0);

        let target = OpaqueColor::from_rgb8(255, 0, 0);
        let hsl = target.convert::<Hsl>();
        println!("{}", hsl.components[1]);
        assert_eq!(hsl.components[1], 100.0);
    }
}
