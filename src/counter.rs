use std::ops::Range;

use image::{Rgb, RgbImage};

pub(crate) trait Counter {
    fn contains(&self, target: &f32) -> bool;
    fn start(&self) -> f32;
    fn end(&self) -> f32;
    fn count(&self) -> u128;
    fn count_up(&mut self);
}

pub(crate) trait Filter<T> {
    fn contains(&self, target: &T) -> bool;
    fn to_target(pixel: &Rgb<u8>) -> T;

    fn hue_filter(start_hue: &Option<u16>, end_hue: &Option<u16>) -> Option<Angle> {
        if let Some(end) = *end_hue {
            let end = end as f32;
            let start = start_hue.unwrap_or(0) as f32;
            let width = if end > start {
                end - start
            } else {
                end + 360.0 - start
            };
            Some(Angle::new(start, width))
        } else {
            None
        }
    }

    fn filter_value<F>(&self, pixel: &Rgb<u8>, get_value: F) -> Option<f32>
    where
        F: FnOnce(&T) -> f32,
    {
        let target = Self::to_target(pixel);

        if self.contains(&target) {
            Some(get_value(&target))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Angle {
    range_1: Range<f32>,
    range_2: Option<Range<f32>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AngleCounter {
    start: f32,
    width: f32,
    angle: Angle,
    count: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PercentageCounter {
    start: f32,
    end: f32,
    width: f32,
    range: Range<f32>,
    count: u128,
}

impl Angle {
    pub fn new(start: f32, width: f32) -> Angle {
        let start = rotate_value(start);
        let width = rotate_value(width);

        let (range_1, range_2) = if start + width >= 360.0 {
            let remain = start + width - 360.0;
            (start..360.0, Some(0.0..remain))
        } else {
            (start..start + width, None)
        };

        Angle { range_1, range_2 }
    }

    pub fn contains(&self, target: &f32) -> bool {
        self.range_1.contains(target)
            | self
                .range_2
                .as_ref()
                .map(|v| v.contains(target))
                .unwrap_or(false)
    }
}

impl AngleCounter {
    pub fn new(start: f32, width: f32) -> AngleCounter {
        let start = rotate_value(start);
        let width = rotate_value(width);

        let angle = Angle::new(start, width);

        AngleCounter {
            start,
            width,
            angle,
            count: 0,
        }
    }
}

impl Counter for AngleCounter {
    fn contains(&self, target: &f32) -> bool {
        self.angle.contains(target)
    }

    fn start(&self) -> f32 {
        self.start
    }

    fn end(&self) -> f32 {
        rotate_value(self.start + self.width)
    }

    fn count(&self) -> u128 {
        self.count
    }

    fn count_up(&mut self) {
        self.count += 1;
    }
}

impl PercentageCounter {
    pub fn new(start: f32, width: f32) -> PercentageCounter {
        let end = start + width;
        let range = start..end;

        PercentageCounter {
            start,
            end,
            width,
            range,
            count: 0,
        }
    }
}

impl Counter for PercentageCounter {
    fn contains(&self, target: &f32) -> bool {
        self.range.contains(target)
    }

    fn start(&self) -> f32 {
        self.start
    }

    fn end(&self) -> f32 {
        self.end
    }

    fn count(&self) -> u128 {
        self.count
    }

    fn count_up(&mut self) {
        self.count += 1;
    }
}

pub(crate) fn count_by_func_with_filter<T, C, F, G>(
    rgb_image: &RgbImage,
    counters: &mut [C],
    filter: F,
    get_value: G,
) -> f64
where
    C: Counter,
    F: Filter<T>,
    G: Fn(&T) -> f32,
{
    let pixels = rgb_image.pixels();
    let mut total_value = 0.0;
    let mut total_pixel = 0;

    for pixel in pixels {
        let target = filter.filter_value(pixel, &get_value);

        if let Some(t) = target {
            if let Some(counter) = counters.iter_mut().find(|c| c.contains(&t)) {
                total_value += t as f64;
                total_pixel += 1;
                counter.count_up();
            }
        }
    }

    total_value / total_pixel as f64
}

pub(crate) fn create_counters<C, B>(divisor: u16, start: f32, max: f32, builder: B) -> Vec<C>
where
    B: Fn(f32, f32) -> C,
{
    let mut counters = Vec::new();
    if divisor == 0 {
        return counters;
    }

    let quotient = max / divisor as f32;
    let mut target_value = 0.0;

    for _i in 0..divisor - 1 {
        let start_value = start + target_value;
        let counter = builder(start_value, quotient);
        counters.push(counter);
        target_value += quotient;
    }

    let remain = max - target_value;
    let start_value = start + target_value;
    let counter = builder(start_value, remain + f32::EPSILON);
    counters.push(counter);

    counters
}

pub(crate) fn print_count<T>(vec: &[T], width: u32, height: u32, filterd_avr: f64)
where
    T: Counter,
{
    let total_pixel = ((width * height) as f32).max(1.0);

    for counter in vec.iter() {
        let ratio = counter.count() as f32 / total_pixel * 100.0;

        println!(
            "{0:>6.2} -> {1:>6.2} : {2:>6.2}% ({3:>10} px)",
            counter.start(),
            counter.end(),
            ratio,
            counter.count()
        )
    }
    println!();
    println!(" avr : {0:>8.4}", filterd_avr);
}

fn rotate_value(raw_value: f32) -> f32 {
    if (0.0..360.0).contains(&raw_value.abs()) {
        raw_value.abs()
    } else {
        raw_value.abs() - 360.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod angle_counter {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn check_value() {
                let result = AngleCounter::new(350.0, 50.0);
                assert_eq!(result.start, 350.0);
                assert_eq!(result.width, 50.0);
                assert_eq!(result.angle.range_1, 350.0..360.0);
                assert_eq!(result.angle.range_2, Some(0.0..40.0));

                let result = AngleCounter::new(50.0, 40.0);
                assert_eq!(result.start, 50.0);
                assert_eq!(result.width, 40.0);
                assert_eq!(result.angle.range_1, 50.0..90.0);
                assert_eq!(result.angle.range_2, None);
            }
        }

        mod contains {
            use super::*;

            #[test]
            fn checking_value() {
                let result = AngleCounter::new(350.0, 50.0);
                assert!(result.contains(&359.0));
                assert!(result.contains(&0.1));
                assert!(!result.contains(&349.0));
                assert!(!result.contains(&51.0));

                let result = AngleCounter::new(50.0, 40.0);
                assert!(result.contains(&51.0));
                assert!(result.contains(&89.0));
                assert!(!result.contains(&49.0));
                assert!(!result.contains(&90.1));
            }
        }

        mod end {
            use super::*;

            #[test]
            fn checking_value() {
                let result = AngleCounter::new(350.0, 50.0);
                assert_eq!(result.end(), 40.0);

                let result = AngleCounter::new(50.0, 40.0);
                assert_eq!(result.end(), 90.0);
            }
        }
    }

    mod create_counters {
        use super::*;

        #[test]
        fn checking_value_angle() {
            let result = create_counters(7, 0.0, 360.0, AngleCounter::new);
            assert_eq!(result.len(), 7);

            let result = create_counters(7, 350.0, 360.0, AngleCounter::new);
            assert_eq!(result.len(), 7);

            let result = create_counters(1, 180.0, 360.0, AngleCounter::new);
            assert_eq!(result.len(), 1);

            let result = create_counters(360, 180.0, 360.0, AngleCounter::new);
            assert_eq!(result.len(), 360);
        }

        #[test]
        fn checking_value_percentage() {
            let result = create_counters(7, 0.0, 1.0, PercentageCounter::new);
            assert_eq!(result.len(), 7);

            let result = create_counters(1, 0.0, 1.0, PercentageCounter::new);
            assert_eq!(result.len(), 1);

            let result = create_counters(100, 0.0, 1.0, PercentageCounter::new);
            assert_eq!(result.len(), 100);
        }
    }
}
