use std::{collections::HashMap, fs::File, io::Write};

use image::RgbImage;
use serde::Serialize;

use crate::{
    config::OutputArgs,
    counter::{Counter, Filter},
    error::PxTallyError,
};

#[derive(Debug, Serialize)]
struct OutputJson {
    tool_name: String,
    tool_version: String,
    schema_version: u32,
    image: ImageData,
    analysis: AnalysisData,
}

#[derive(Debug, Serialize)]
struct ImageData {
    width: u32,
    height: u32,
    pixels: u128,
}

#[derive(Debug, Serialize)]
struct AnalysisData {
    color_space: String,
    component: String,
    interval_type: String,
    ranges: HashMap<String, FilterRange>,
    bins: Vec<BinData>,
    stats: Stats,
}

#[derive(Debug, Serialize)]
struct FilterRange {
    start: f32,
    end: f32,
}

#[derive(Debug, Serialize)]
struct BinData {
    range_start: f32,
    range_end: f32,
    ratio: f64,
    pixel_count: u128,
}

#[derive(Debug, Serialize)]
struct Stats {
    average: f64,
}

impl OutputJson {
    pub fn new<C, F, T>(
        color_space_name: &str,
        component_name: &str,
        counters: &[C],
        rgb_image: &RgbImage,
        filter: &F,
        filtered_totals: (f64, u128),
    ) -> OutputJson
    where
        C: Counter,
        F: Filter<T>,
    {
        let tool_name = env!("CARGO_BIN_NAME").to_string();
        let tool_version = env!("CARGO_PKG_VERSION").to_string();
        let schema_version = 1;

        let width = rgb_image.width();
        let height = rgb_image.height();
        let pixels = (width as u128 * height as u128).max(1);

        let image = ImageData::new(width, height, pixels);

        let analysis = AnalysisData::new(
            color_space_name,
            component_name,
            pixels,
            filter,
            counters,
            filtered_totals,
        );

        OutputJson {
            tool_name,
            tool_version,
            schema_version,
            image,
            analysis,
        }
    }
}

impl ImageData {
    pub fn new(width: u32, height: u32, pixels: u128) -> ImageData {
        ImageData {
            width,
            height,
            pixels,
        }
    }
}

impl AnalysisData {
    pub fn new<C, F, T>(
        color_space_name: &str,
        component_name: &str,
        total_pixel: u128,
        filter: &F,
        counters: &[C],
        filtered_tolals: (f64, u128),
    ) -> AnalysisData
    where
        C: Counter,
        F: Filter<T>,
    {
        let color_space = color_space_name.to_string();
        let component = component_name.to_string();

        let mut ranges = HashMap::new();
        if let Some(hue_range) = filter.hue_filter() {
            ranges.insert(
                "hue".to_string(),
                FilterRange {
                    start: hue_range.start(),
                    end: hue_range.end(),
                },
            );
        }

        let interval_type = "[start,end)".to_string();

        let bins: Vec<BinData> = counters
            .iter()
            .map(|c| BinData::new(c, total_pixel))
            .collect();

        let stats = Stats::new(filtered_tolals);

        AnalysisData {
            color_space,
            component,
            ranges,
            interval_type,
            bins,
            stats,
        }
    }
}

impl BinData {
    pub fn new<C>(counter: &C, total_pixel: u128) -> BinData
    where
        C: Counter,
    {
        let range_start = counter.start();
        let range_end = counter.end();
        let pixel_count = counter.count();
        let ratio = pixel_count as f64 / total_pixel as f64;

        BinData {
            range_start,
            range_end,
            ratio,
            pixel_count,
        }
    }
}

impl Stats {
    pub fn new((filtered_total_value, filtered_total_pixel): (f64, u128)) -> Stats {
        let average = filtered_total_value / filtered_total_pixel as f64;

        Stats { average }
    }
}

pub(crate) fn output<C, F, T>(
    color_space_name: &str,
    component_name: &str,
    counters: &[C],
    rgb_image: &RgbImage,
    filter: &F,
    output_args: &OutputArgs,
    filtered_totals: (f64, u128),
) -> Result<(), PxTallyError>
where
    C: Counter,
    F: Filter<T>,
{
    output_json(
        output_args,
        color_space_name,
        component_name,
        counters,
        rgb_image,
        filter,
        filtered_totals,
    )?;

    if !output_args.no_print {
        output_stdout(
            color_space_name,
            component_name,
            counters,
            rgb_image.width(),
            rgb_image.height(),
            filter,
            filtered_totals,
        );
    }

    Ok(())
}

fn output_json<C, F, T>(
    output_args: &OutputArgs,
    color_space_name: &str,
    component_name: &str,
    counters: &[C],
    rgb_image: &RgbImage,
    filter: &F,
    filtered_totals: (f64, u128),
) -> Result<(), PxTallyError>
where
    C: Counter,
    F: Filter<T>,
{
    if output_args.json || output_args.json_output.is_some() {
        let json_struct = OutputJson::new(
            color_space_name,
            component_name,
            counters,
            rgb_image,
            filter,
            filtered_totals,
        );
        let json_string = serde_json::to_string(&json_struct)?;

        if output_args.json {
            println!("{}", json_string);
        }

        if let Some(path) = output_args.json_output.as_ref() {
            let mut file = File::create(path)?;
            file.write_all(json_string.as_bytes())?;
        }
    }

    Ok(())
}

fn output_stdout<C, F, T>(
    color_space_name: &str,
    component_name: &str,
    vec: &[C],
    width: u32,
    height: u32,
    filter: &F,
    (filtered_total_value, filtered_total_pixel): (f64, u128),
) where
    C: Counter,
    F: Filter<T>,
{
    let total_pixel = ((width * height) as f32).max(1.0);

    println!("{} {}", color_space_name, component_name);

    if let Some(hue_filter) = filter.hue_filter() {
        println!(
            "hue range: {:>6.2} - {:>6.2}",
            hue_filter.start(),
            hue_filter.end()
        );
    }

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
    let filtered_avr = filtered_total_value / filtered_total_pixel as f64;
    println!();
    println!(" avr : {0:>8.4}", filtered_avr);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod output_json {
        use std::ops::Range;

        use image::Rgb;

        use crate::counter::{
            count_by_func_with_filter, create_counters, Angle, PercentageCounter,
        };

        use super::*;

        struct TestFilter {
            r_range: Option<Range<f32>>,
            hue_range: Option<Angle>,
        }

        impl TestFilter {
            pub fn new(r_range: Option<Range<f32>>, hue_range: Option<Angle>) -> TestFilter {
                TestFilter { r_range, hue_range }
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
                self.hue_range.as_ref()
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

        fn test_get_value_b(rgb: &Rgb<u8>) -> f32 {
            rgb.0[2] as f32
        }

        #[test]
        fn checking_value() {
            assert!(checking_case_001().is_ok());
            assert!(checking_case_002().is_ok());
        }

        fn checking_case_001() -> Result<String, serde_json::Error> {
            let case = case_rgb_image();
            let mut counters = create_counters(10, 0.0, 255.0, PercentageCounter::new);
            let filter = TestFilter::new(None, None);
            let filtererd_totals =
                count_by_func_with_filter(&case, &mut counters, &filter, test_get_value_b);

            let output_json =
                OutputJson::new("rgb", "b", &counters, &case, &filter, filtererd_totals);

            serde_json::to_string(&output_json)
        }

        fn checking_case_002() -> Result<String, serde_json::Error> {
            let case = case_rgb_image();
            let mut counters = create_counters(10, 0.0, 255.0, PercentageCounter::new);
            let filter = TestFilter::new(
                Some(Range {
                    start: 0.0,
                    end: 20.0_f32.next_up(),
                }),
                Some(Angle::new(0.0, 20.0_f32.next_up())),
            );
            let filtererd_totals =
                count_by_func_with_filter(&case, &mut counters, &filter, test_get_value_b);

            let output_json =
                OutputJson::new("rgb", "b", &counters, &case, &filter, filtererd_totals);

            serde_json::to_string(&output_json)
        }
    }
}
