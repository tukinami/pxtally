use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use image::RgbImage;
use serde::Serialize;

use crate::{
    config::OutputArgs,
    counter::{Counter, Filter},
    error::PxTallyError,
};

const OUTPUT_JSON_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Serialize)]
struct OutputJson {
    tool_name: String,
    tool_version: String,
    schema_version: u32,
    analyzed_at: u64,
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
    median: f64,
    standard_deviation: f64,
    extracted_total_value: f64,
    extracted_total_pixel: u128,
}

impl OutputJson {
    pub fn new<C, F, T>(
        color_space_name: &str,
        component_name: &str,
        counters: &[C],
        rgb_image: &RgbImage,
        filter: &F,
        extracted_totals: (f64, u128),
    ) -> Result<OutputJson, PxTallyError>
    where
        C: Counter,
        F: Filter<T>,
    {
        let tool_name = env!("CARGO_BIN_NAME").to_string();
        let tool_version = env!("CARGO_PKG_VERSION").to_string();
        let schema_version = OUTPUT_JSON_SCHEMA_VERSION;
        let system_time = std::time::SystemTime::now();
        let analyzed_at = system_time.duration_since(std::time::UNIX_EPOCH)?.as_secs();

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
            extracted_totals,
        );

        Ok(OutputJson {
            tool_name,
            tool_version,
            analyzed_at,
            schema_version,
            image,
            analysis,
        })
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
        extracted_tolals: (f64, u128),
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

        let stats = Stats::new(extracted_tolals, &bins);

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
    pub fn new(
        (extracted_total_value, extracted_total_pixel): (f64, u128),
        bins: &[BinData],
    ) -> Stats {
        let average = extracted_total_value / extracted_total_pixel as f64;
        let median = Stats::calc_median(bins, extracted_total_pixel);
        let standard_deviation =
            Stats::calc_standard_deviation(bins, extracted_total_pixel, average);

        Stats {
            average,
            median,
            standard_deviation,
            extracted_total_value,
            extracted_total_pixel,
        }
    }

    fn calc_median(bins: &[BinData], extracted_total_pixel: u128) -> f64 {
        let half = extracted_total_pixel as f64 / 2.0;
        bins.iter()
            .scan(0u128, |cumulative, bin| {
                *cumulative += bin.pixel_count;
                Some((*cumulative, bin))
            })
            .find(|(cumulative, _)| *cumulative as f64 >= half)
            .map(|(cumulative, bin)| {
                let prev = cumulative - bin.pixel_count;
                let t = (half - prev as f64) / bin.pixel_count as f64;
                bin.range_start as f64 + t * (bin.range_end as f64 - bin.range_start as f64)
            })
            .unwrap_or(0.0)
    }

    fn calc_standard_deviation(bins: &[BinData], extracted_total_pixel: u128, average: f64) -> f64 {
        let variance = bins
            .iter()
            .map(|b| {
                let center = (b.range_start + b.range_end) as f64 / 2.0;
                let diff = center - average;
                diff.powf(2.0) * b.pixel_count as f64
            })
            .sum::<f64>()
            / extracted_total_pixel as f64;
        variance.sqrt()
    }
}

pub(crate) fn confirm_and_run<P, F>(path: P, f: F) -> Result<(), PxTallyError>
where
    P: AsRef<Path>,
    F: FnOnce() -> Result<(), PxTallyError>,
{
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    let mut input = String::new();

    loop {
        println!(
            "{}",
            t!("output.confirm.q.1", path = path.as_ref().display())
        );
        print!("{} [yes/no]: ", t!("output.confirm.q.2"));
        stdout.flush()?;

        input.clear();
        stdin.read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return f(),
            "no" | "n" => return Ok(()),
            _ => println!("{}", t!("output.confirm.a")),
        }
    }
}

pub(crate) fn output<C, F, T>(
    color_space_name: &str,
    component_name: &str,
    counters: &[C],
    rgb_image: &RgbImage,
    filter: &F,
    output_args: &OutputArgs,
    extracted_totals: (f64, u128),
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
        extracted_totals,
    )?;

    if !output_args.no_print {
        output_stdout(
            color_space_name,
            component_name,
            counters,
            rgb_image.width(),
            rgb_image.height(),
            filter,
            extracted_totals,
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
    extracted_totals: (f64, u128),
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
            extracted_totals,
        )?;
        let json_string = serde_json::to_string(&json_struct)?;

        if output_args.json {
            println!("{}", json_string);
        }

        if let Some(path) = output_args.json_output.as_ref() {
            if path.exists() && !output_args.force {
                confirm_and_run(path, || write_json(path, json_string.as_str()))?;
            } else {
                write_json(path, json_string.as_str())?;
            }
        }
    }

    Ok(())
}

fn write_json(path: &PathBuf, json_string: &str) -> Result<(), PxTallyError> {
    let mut file = File::create(path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn output_stdout<C, F, T>(
    color_space_name: &str,
    component_name: &str,
    vec: &[C],
    width: u32,
    height: u32,
    filter: &F,
    (extracted_total_value, extracted_total_pixel): (f64, u128),
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
            "{0:>7.2} -> {1:>7.2} : {2:>6.2}% ({3:>10} px)",
            counter.start(),
            counter.end(),
            ratio,
            counter.count()
        )
    }
    let extracted_avr = extracted_total_value / extracted_total_pixel as f64;
    println!();
    println!(" avr : {0:>9.4}", extracted_avr);
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
                OutputJson::new("rgb", "b", &counters, &case, &filter, filtererd_totals).unwrap();

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
                OutputJson::new("rgb", "b", &counters, &case, &filter, filtererd_totals).unwrap();

            serde_json::to_string(&output_json)
        }
    }

    mod stats {
        use super::*;

        fn case_bins_and_total_pixel(range: u128) -> (Vec<BinData>, u128) {
            let mut case = Vec::new();
            let total_pixel = (0..range).step_by(2).fold(0, |acc, v| (v * 10) + acc);

            for i in (0..range).step_by(2) {
                let range_start = i as f32;
                let range_end = range_start + 2.0;
                let pixel_count = i * 10;
                let ratio = pixel_count as f64 / total_pixel as f64;

                let bin = BinData {
                    range_start,
                    range_end,
                    ratio,
                    pixel_count,
                };
                case.push(bin);
            }

            (case, total_pixel)
        }

        mod median {
            use super::*;
            pub fn calc_median_temp(bins: &[BinData], _total_pixel: u128) -> f64 {
                let mut values: Vec<f32> = bins
                    .iter()
                    .map(|v| {
                        let mut vec = Vec::new();
                        let value = (v.range_start + v.range_end) / 2.0;
                        for _i in 0..v.pixel_count {
                            vec.push(value);
                        }
                        vec
                    })
                    .flatten()
                    .collect();
                values.sort_by(|a, b| a.total_cmp(b));

                let half_index = values.len() / 2;

                if values.len() % 2 == 0 {
                    let first = values[half_index];
                    let second = values[half_index + 1];

                    ((first + second) / 2.0) as f64
                } else {
                    values[half_index + 1] as f64
                }
            }

            #[test]
            fn checking_value() {
                let case = [10, 20, 30, 40, 50];
                let results_01: Vec<f64> = case
                    .iter()
                    .map(|v| {
                        let (bins, total_pixel) = case_bins_and_total_pixel(*v);
                        Stats::calc_median(&bins, total_pixel)
                    })
                    .collect();
                let results_02: Vec<f64> = case
                    .iter()
                    .map(|v| {
                        let (bins, total_pixel) = case_bins_and_total_pixel(*v);
                        calc_median_temp(&bins, total_pixel)
                    })
                    .collect();
                println!("results_01: {:?}", results_01);
                println!("results_02: {:?}", results_02);

                let results_01_avr =
                    results_01.iter().fold(0.0, |acc, v| acc + v) / case.len() as f64;
                let results_02_avr =
                    results_02.iter().fold(0.0, |acc, v| acc + v) / case.len() as f64;
                let diff = results_01_avr - results_02_avr;

                assert!((-1.0..1.0).contains(&diff));
            }
        }

        mod standard_deviation {
            use super::*;

            fn case_bins_and_total_pixel_std_div_0(range: u128) -> (Vec<BinData>, u128) {
                let mut case = Vec::new();
                let total_pixel = (0..range).step_by(2).fold(0, |acc, v| (v * 10) + acc);
                let total_value = (0..range).step_by(2).fold(0, |acc, v| v + 1 + acc);
                let average = (total_value as f64 / total_pixel as f64) as f32;

                for i in (0..range).step_by(2) {
                    let range_start = i as f32;
                    let range_end = range_start + 2.0;
                    let pixel_count = if (range_start..range_end).contains(&average) {
                        total_pixel
                    } else {
                        0
                    };
                    let ratio = pixel_count as f64 / total_pixel as f64;

                    let bin = BinData {
                        range_start,
                        range_end,
                        ratio,
                        pixel_count,
                    };
                    case.push(bin);
                }

                (case, total_pixel)
            }

            #[test]
            fn checking_value_std_div_0() {
                let case = [10, 20, 30, 40, 50];
                let results_01: Vec<f64> = case
                    .iter()
                    .map(|v| {
                        let (bins, total_pixel) = case_bins_and_total_pixel_std_div_0(*v);
                        let total_values = bins
                            .iter()
                            .fold(0.0, |acc, v| (v.range_start - v.range_end) + acc)
                            as f64;
                        let average = total_values / total_pixel as f64;
                        Stats::calc_standard_deviation(&bins, total_pixel, average)
                    })
                    .collect();

                println!("results_01: {:?}", results_01);

                let results_01_avr =
                    results_01.iter().fold(0.0, |acc, v| acc + v) / case.len() as f64;

                assert!((-2.0..2.0).contains(&results_01_avr));
            }

            #[test]
            fn checking_value_std_div_normal() {
                let case = [10, 20, 30, 40, 50];
                let results_01: Vec<f64> = case
                    .iter()
                    .map(|v| {
                        let (bins, total_pixel) = case_bins_and_total_pixel(*v);
                        let total_values = bins
                            .iter()
                            .fold(0.0, |acc, v| (v.range_start - v.range_end) + acc)
                            as f64;
                        let average = total_values / total_pixel as f64;
                        Stats::calc_standard_deviation(&bins, total_pixel, average)
                    })
                    .collect();

                println!("results_01: {:?}", results_01);

                let results_01_avr =
                    results_01.iter().fold(0.0, |acc, v| acc + v) / case.len() as f64;

                assert!(!(-2.0..2.0).contains(&results_01_avr));
            }
        }
    }
}
