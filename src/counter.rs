use std::{collections::HashMap, ops::Range};

use image::{Rgb, RgbImage};

pub(crate) trait Counter {
    fn contains(&self, target: &f32) -> bool;
    fn start(&self) -> f32;
    fn end(&self) -> f32;
    fn count(&self) -> u128;
    fn count_add(&mut self, value: u128);
}

pub(crate) trait Filter<T> {
    fn contains(&self, target: &T) -> bool;
    fn to_target(pixel: &Rgb<u8>) -> T;
    fn hue_filter(&self) -> Option<&Angle>;

    fn create_hue_filter(start_hue: &Option<u16>, end_hue: &Option<u16>) -> Option<Angle> {
        if let Some(end) = *end_hue {
            let end = end as f32;
            let start = start_hue.unwrap_or(0) as f32;
            Some(Angle::new(start, end))
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
    angle: Angle,
    count: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PercentageCounter {
    start: f32,
    end: f32,
    range: Range<f32>,
    count: u128,
}

impl Angle {
    pub fn new(start: f32, end: f32) -> Angle {
        let start = rotate_value(start);
        let end = rotate_value(end);

        let (range_1, range_2) = if start > end {
            (start..360.0, Some(0.0..end))
        } else {
            (start..end, None)
        };

        Angle { range_1, range_2 }
    }

    pub fn contains(&self, target: &f32) -> bool {
        let target = rotate_value(*target);

        self.range_1.contains(&target)
            | self
                .range_2
                .as_ref()
                .map(|v| v.contains(&target))
                .unwrap_or(false)
    }

    pub fn start(&self) -> f32 {
        self.range_1.start
    }

    pub fn end(&self) -> f32 {
        self.range_2
            .as_ref()
            .map(|v| v.end)
            .unwrap_or(self.range_1.end)
    }
}

impl AngleCounter {
    pub fn new(start: f32, end: f32) -> AngleCounter {
        let angle = Angle::new(start, end);

        AngleCounter { angle, count: 0 }
    }
}

impl Counter for AngleCounter {
    fn contains(&self, target: &f32) -> bool {
        self.angle.contains(target)
    }

    fn start(&self) -> f32 {
        self.angle.start()
    }

    fn end(&self) -> f32 {
        self.angle.end()
    }

    fn count(&self) -> u128 {
        self.count
    }

    fn count_add(&mut self, value: u128) {
        self.count += value
    }
}

impl PercentageCounter {
    pub fn new(start: f32, end: f32) -> PercentageCounter {
        let range = start..end;

        PercentageCounter {
            start,
            end,
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

    fn count_add(&mut self, value: u128) {
        self.count += value
    }
}

pub(crate) fn count_by_func_with_filter<T, C, F, G>(
    rgb_image: &RgbImage,
    counters: &mut [C],
    filter: &F,
    get_value: G,
) -> (f64, u128)
where
    C: Counter,
    F: Filter<T>,
    G: Fn(&T) -> f32,
{
    let pixels = rgb_image.pixels();
    let mut total_value = 0.0;
    let mut total_pixel = 0;
    let mut rgb_count: HashMap<Rgb<u8>, u128> = HashMap::new();

    for pixel in pixels {
        rgb_count.entry(*pixel).and_modify(|c| *c += 1).or_insert(1);
    }

    for (pixel, pixel_count) in rgb_count.iter() {
        let target = filter.filter_value(pixel, &get_value);

        if let Some(t) = target {
            if let Some(counter) = counters.iter_mut().find(|c| c.contains(&t)) {
                total_value += t as f64 * *pixel_count as f64;
                total_pixel += *pixel_count;
                counter.count_add(*pixel_count);
            }
        }
    }

    (total_value, total_pixel)
}

pub(crate) fn create_counters<C, B>(divisor: u16, start: f32, width: f32, builder: B) -> Vec<C>
where
    B: Fn(f32, f32) -> C,
{
    let mut counters = Vec::new();
    if divisor == 0 {
        return counters;
    }

    let start = start as f64;
    let quotient = width as f64 / divisor as f64;

    for i in 0..divisor - 1 {
        let start_value = quotient.mul_add(i as f64, start);
        let end_value = quotient.mul_add(i as f64 + 1.0, start);
        let counter = builder(start_value as f32, end_value as f32);
        counters.push(counter);
    }
    let start_value = quotient.mul_add((divisor - 1) as f64, start);
    let end_value = start + width as f64;
    let counter = builder(start_value as f32, (end_value as f32).next_up());
    counters.push(counter);

    counters
}

fn rotate_value(raw_value: f32) -> f32 {
    if (0.0..360.0).contains(&raw_value) {
        raw_value
    } else if raw_value < 0.0 {
        360.0 + raw_value
    } else {
        raw_value - 360.0
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
                let result = AngleCounter::new(350.0, 400.0);
                assert_eq!(result.start(), 350.0);
                assert_eq!(result.angle.range_1, 350.0..360.0);
                assert_eq!(result.angle.range_2, Some(0.0..40.0));

                let result = AngleCounter::new(50.0, 90.0);
                assert_eq!(result.start(), 50.0);
                assert_eq!(result.angle.range_1, 50.0..90.0);
                assert_eq!(result.angle.range_2, None);
            }
        }

        mod contains {
            use super::*;

            #[test]
            fn checking_value() {
                let result = AngleCounter::new(350.0, 400.0);
                assert!(result.contains(&359.0));
                assert!(result.contains(&0.1));
                assert!(!result.contains(&349.0));
                assert!(!result.contains(&41.0));

                let result = AngleCounter::new(50.0, 90.0);
                assert!(result.contains(&51.0));
                assert!(result.contains(&89.0));
                assert!(!result.contains(&49.0));
                assert!(!result.contains(&90.1));

                let result = AngleCounter::new(350.0, 400.0_f32.next_up());
                assert!(result.contains(&40.0));

                let result = AngleCounter::new(50.0, 90.0_f32.next_up());
                assert!(result.contains(&90.0));
            }
        }

        mod end {
            use super::*;

            #[test]
            fn checking_value() {
                let result = AngleCounter::new(350.0, 400.0);
                assert_eq!(result.end(), 40.0);

                let result = AngleCounter::new(50.0, 90.0);
                assert_eq!(result.end(), 90.0);

                let result = AngleCounter::new(350.0, 360.0_f32.next_up());
                assert_eq!(result.end(), rotate_value(360.0_f32.next_up()));
            }
        }
    }

    mod percentage_counter {
        use super::*;

        mod contains {
            use super::*;

            #[test]
            fn checking_value() {
                let result = PercentageCounter::new(0.0, 1.0f32.next_up());
                assert!(result.contains(&1.0));
            }
        }
    }

    mod count_by_func_with_iter {
        use super::*;

        struct TestFilter {
            r_range: Option<Range<f32>>,
        }

        impl TestFilter {
            pub fn new(r_range: Option<Range<f32>>) -> TestFilter {
                TestFilter { r_range }
            }
        }

        impl Filter<Rgb<u8>> for TestFilter {
            fn contains(&self, target: &Rgb<u8>) -> bool {
                self.r_range
                    .as_ref()
                    .map(|v| v.contains(&(target.0[0] as f32)))
                    .unwrap_or(true)
            }
            fn to_target(pixel: &Rgb<u8>) -> Rgb<u8> {
                *pixel
            }

            fn hue_filter(&self) -> Option<&Angle> {
                None
            }
        }

        fn case_rgb_image() -> RgbImage {
            let mut image = RgbImage::new(2, 3);
            for index_r in 0..3 {
                let r = 20 * index_r as u8;
                for index_b in 0..2 {
                    let b = 30 * index_b as u8;

                    let pixel = Rgb::from([r, 0, b]);
                    image.put_pixel(index_b, index_r, pixel);
                }
            }

            image
        }

        #[test]
        fn checking_value() {
            let case = case_rgb_image();
            fn test_get_value_b(rgb: &Rgb<u8>) -> f32 {
                rgb.0[2] as f32
            }

            let mut counters = create_counters(10, 0.0, 255.0, PercentageCounter::new);
            let filter = TestFilter::new(None);

            let (filtered_total_value, filtered_total_pixel) =
                count_by_func_with_filter(&case, &mut counters, &filter, test_get_value_b);
            let filtered_avarage = filtered_total_value / filtered_total_pixel as f64;
            assert_eq!(filtered_avarage, 15.0);

            let value_0_count = counters
                .iter()
                .filter(|c| c.contains(&0.0))
                .fold(0, |acc, c| c.count + acc);
            assert_eq!(value_0_count, 3);
            let value_30_count = counters
                .iter()
                .filter(|c| c.contains(&30.0))
                .fold(0, |acc, c| c.count + acc);
            assert_eq!(value_30_count, 3);

            let total_pixel = counters.iter().fold(0, |acc, c| c.count + acc);
            assert_eq!(total_pixel, 6);

            let mut counters = create_counters(10, 0.0, 255.0, PercentageCounter::new);
            let r_range = 0.0..20.0f32.next_up();
            assert!(r_range.contains(&20.0));
            let filter = TestFilter::new(Some(r_range));

            let (filtered_total_value, filtered_total_pixel) =
                count_by_func_with_filter(&case, &mut counters, &filter, test_get_value_b);
            let filtered_avarage = filtered_total_value / filtered_total_pixel as f64;
            assert_eq!(filtered_avarage, 15.0);

            let value_0_count = counters
                .iter()
                .filter(|c| c.contains(&0.0))
                .fold(0, |acc, c| c.count + acc);
            assert_eq!(value_0_count, 2);
            let value_30_count = counters
                .iter()
                .filter(|c| c.contains(&30.0))
                .fold(0, |acc, c| c.count + acc);
            assert_eq!(value_30_count, 2);

            let total_pixel = counters.iter().fold(0, |acc, c| c.count + acc);
            assert_eq!(total_pixel, 4);
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
            assert!(result.last().map(|v| v.contains(&1.0)).unwrap());

            let result = create_counters(1, 0.0, 1.0, PercentageCounter::new);
            assert_eq!(result.len(), 1);

            let result = create_counters(100, 0.0, 1.0, PercentageCounter::new);
            assert_eq!(result.len(), 100);
        }
    }

    mod rotate_value {
        use super::*;

        #[test]
        fn checking_value() {
            assert_eq!(rotate_value(0.0), 0.0);
            assert_eq!(rotate_value(359.9), 359.9);
            assert_eq!(rotate_value(-10.0), 350.0);
            assert_eq!(rotate_value(380.0), 20.0);
        }
    }
}
