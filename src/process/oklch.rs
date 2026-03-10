use bigcolor::color_space::OKLCH;
use image::{Rgb, RgbImage};

use crate::{
    config::OklchCommands,
    counter::{
        count_by_func_with_filter, create_counters, print_count, Angle, AngleCounter, Filter,
        PercentageCounter,
    },
    process::{load_image, ProcessError},
};

struct OklchFilter {
    hue: Option<Angle>,
}

impl OklchFilter {
    pub fn new(start_hue: &Option<u16>, end_hue: &Option<u16>) -> OklchFilter {
        let hue = <OklchFilter as Filter<OKLCH>>::hue_filter(start_hue, end_hue);

        OklchFilter { hue }
    }
}

impl Filter<OKLCH> for OklchFilter {
    fn contains(&self, target: &OKLCH) -> bool {
        self.hue
            .as_ref()
            .map(|v| v.contains(&target.h))
            .unwrap_or(true)
    }

    fn to_target(pixel: &Rgb<u8>) -> OKLCH {
        let big_color = bigcolor::BigColor::from_rgb(pixel.0[0], pixel.0[1], pixel.0[2], 1.0);
        big_color.to_oklch()
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
    // why `1.000001`, See `self::tests::checking_value()`.
    let mut counters = create_counters(divisor, 0.0, 1.000001, PercentageCounter::new);

    let filter = OklchFilter::new(&None, &None);

    let filterd_avr =
        count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_lightness);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filterd_avr,
    );
}

fn process_chroma(
    rgb_image: &RgbImage,
    divisor: u16,
    start_hue: &Option<u16>,
    end_hue: &Option<u16>,
) {
    let mut counters = create_counters(divisor, 0.0, 0.4, PercentageCounter::new);

    let filter = OklchFilter::new(start_hue, end_hue);

    let filterd_avr = count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_chroma);
    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filterd_avr,
    );
}

fn process_hue(rgb_image: &RgbImage, divisor: u16, start: u16) {
    let start = (start % 360) as f32;
    let mut counters = create_counters(divisor, start, 360.0, AngleCounter::new);

    let filter = OklchFilter::new(&None, &None);

    let filterd_avr = count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_hue);
    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filterd_avr,
    );
}

fn pixel_to_lightness(oklch: &OKLCH) -> f32 {
    oklch.l
}

fn pixel_to_chroma(oklch: &OKLCH) -> f32 {
    oklch.c
}

fn pixel_to_hue(oklch: &OKLCH) -> f32 {
    oklch.h
}

#[cfg(test)]
mod tests {

    #[test]
    fn checking_value() {
        let target = bigcolor::BigColor::from_rgb(255, 255, 255, 1.0);
        let oklch = target.to_oklch();
        println!("{}", oklch.l);
        assert_eq!(oklch.l, 1.000001);

        let target = bigcolor::BigColor::from_rgb(0, 0, 0, 1.0);
        let oklch = target.to_oklch();
        assert_eq!(oklch.l, 0.0);
    }
}
