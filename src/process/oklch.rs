use color::{Oklch, OpaqueColor};
use image::{Rgb, RgbImage};

use crate::{
    config::OklchCommands,
    counter::{
        count_by_func_with_filter, create_counters, print_count, Angle, AngleCounter, Filter,
        PercentageCounter,
    },
    process::{load_image, ProcessError},
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
    hue: Option<Angle>,
}

impl OklchFilter {
    pub fn new(start_hue: &Option<u16>, end_hue: &Option<u16>) -> OklchFilter {
        let hue = <OklchFilter as Filter<OpaqueColor<Oklch>>>::hue_filter(start_hue, end_hue);

        OklchFilter { hue }
    }
}

impl Filter<OpaqueColor<Oklch>> for OklchFilter {
    fn contains(&self, target: &OpaqueColor<Oklch>) -> bool {
        self.hue
            .as_ref()
            .map(|v| v.contains(&target.components[2]))
            .unwrap_or(true)
    }

    fn to_target(pixel: &Rgb<u8>) -> OpaqueColor<Oklch> {
        let color_rgb = OpaqueColor::from_rgb8(pixel.0[0], pixel.0[1], pixel.0[2]);
        color_rgb.convert::<Oklch>()
    }
}

pub(crate) fn process_oklch(command: &OklchCommands) -> Result<(), ProcessError> {
    match &command {
        OklchCommands::Lightness(args) => {
            let rgb_image = load_image(&args.path)?;
            process_lightness(&rgb_image, args.divisor);
        }
        OklchCommands::Chroma(args) => {
            let rgb_image = load_image(&args.path)?;
            process_chroma(&rgb_image, args.divisor, &args.start_hue, &args.end_hue);
        }
        OklchCommands::Hue(args) => {
            let rgb_image = load_image(&args.path)?;
            process_hue(&rgb_image, args.divisor, args.start);
        }
    }
    Ok(())
}

fn process_lightness(rgb_image: &RgbImage, divisor: u16) {
    let mut counters = create_counters(
        divisor,
        constants::LIGHTNESS_MIN,
        constants::LIGHTNESS_MAX,
        PercentageCounter::new,
    );

    let filter = OklchFilter::new(&None, &None);

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

fn process_chroma(
    rgb_image: &RgbImage,
    divisor: u16,
    start_hue: &Option<u16>,
    end_hue: &Option<u16>,
) {
    let mut counters = create_counters(
        divisor,
        constants::CHROMA_MIN,
        constants::CHROMA_MAX,
        PercentageCounter::new,
    );

    let filter = OklchFilter::new(start_hue, end_hue);

    let (filtered_total_value, filtered_total_pixel) =
        count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_chroma);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filtered_total_value,
        filtered_total_pixel,
    );
}

fn process_hue(rgb_image: &RgbImage, divisor: u16, start: u16) {
    let start = (start % 360) as f32;
    let mut counters = create_counters(divisor, start, constants::HUE_MAX, AngleCounter::new);

    let filter = OklchFilter::new(&None, &None);

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
    use color::{Oklch, OpaqueColor};

    #[test]
    fn checking_value() {
        let target = OpaqueColor::from_rgb8(255, 255, 255);
        let oklch = target.convert::<Oklch>();
        println!("{}", oklch.components[0]);
        assert_eq!(oklch.components[0], 1.0);

        let target = OpaqueColor::from_rgb8(0, 0, 0);
        let oklch = target.convert::<Oklch>();
        println!("{}", oklch.components[0]);
        assert_eq!(oklch.components[1], 0.0);
        assert_eq!(oklch.components[0], 0.0);
    }
}
