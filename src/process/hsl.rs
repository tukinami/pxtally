use bigcolor::color_space::HSL;
use image::{Rgb, RgbImage};

use crate::{
    config::HslCommands,
    counter::{
        count_by_func_with_filter, create_counters, print_count, Angle, AngleCounter, Filter,
        PercentageCounter,
    },
    process::{load_image, ProcessError},
};

struct HslFilter {
    hue: Option<Angle>,
}

impl HslFilter {
    pub fn new(start_hue: &Option<u16>, end_hue: &Option<u16>) -> HslFilter {
        let hue = <HslFilter as Filter<HSL>>::hue_filter(start_hue, end_hue);

        HslFilter { hue }
    }
}

impl Filter<HSL> for HslFilter {
    fn contains(&self, target: &HSL) -> bool {
        self.hue
            .as_ref()
            .map(|v| v.contains(&target.h))
            .unwrap_or(true)
    }

    fn to_target(pixel: &Rgb<u8>) -> HSL {
        let big_color = bigcolor::BigColor::from_rgb(pixel.0[0], pixel.0[1], pixel.0[2], 1.0);
        big_color.to_hsl()
    }
}

pub(crate) fn process_hsl(command: &HslCommands) -> Result<(), ProcessError> {
    match &command {
        HslCommands::Hue(args) => {
            let rgb_image = load_image(&args.path)?;
            process_hue(&rgb_image, args.divisor, args.start);
        }
        HslCommands::Saturation(args) => {
            let rgb_image = load_image(&args.path)?;
            process_saturation(&rgb_image, args.divisor);
        }
        HslCommands::Lightness(args) => {
            let rgb_image = load_image(&args.path)?;
            process_lightness(&rgb_image, args.divisor);
        }
    }
    Ok(())
}

fn process_hue(rgb_image: &RgbImage, divisor: u16, start: u16) {
    let start = (start % 360) as f32;
    let mut counters = create_counters(divisor, start, 360.0, AngleCounter::new);

    let filter = HslFilter::new(&None, &None);

    let filterd_avr = count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_hue);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filterd_avr,
    );
}

fn process_saturation(rgb_image: &RgbImage, divisor: u16) {
    let mut counters = create_counters(divisor, 0.0, 1.0, PercentageCounter::new);

    let filter = HslFilter::new(&None, &None);

    let filterd_avr =
        count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_saturation);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filterd_avr,
    );
}

fn process_lightness(rgb_image: &RgbImage, divisor: u16) {
    let mut counters = create_counters(divisor, 0.0, 1.0, PercentageCounter::new);

    let filter = HslFilter::new(&None, &None);

    let filterd_avr =
        count_by_func_with_filter(rgb_image, &mut counters, filter, pixel_to_lightness);

    print_count(
        &counters,
        rgb_image.width(),
        rgb_image.height(),
        filterd_avr,
    );
}

fn pixel_to_hue(hsla: &HSL) -> f32 {
    hsla.h
}

fn pixel_to_saturation(hsla: &HSL) -> f32 {
    hsla.s
}

fn pixel_to_lightness(hsla: &HSL) -> f32 {
    hsla.l
}
