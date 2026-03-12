use std::path::PathBuf;

use image::RgbImage;

use crate::{
    config::OutputArgs,
    counter::{Counter, Filter},
    process::ProcessError,
};

#[derive(Debug)]
struct OutputJson {
    tool_name: String,
    version: String,
    image: ImageData,
    analysis: AnalysisData,
}

#[derive(Debug)]
struct ImageData {
    path: PathBuf,
    width: u32,
    height: u32,
    pixels: u128,
}

#[derive(Debug)]
struct AnalysisData {
    colorspace: String,
    components: String,
    hue_range: Option<HueRange>,
    bins: Vec<BinData>,
}

#[derive(Debug)]
struct HueRange {
    start: f32,
    end: f32,
}

#[derive(Debug)]
struct BinData {
    range_start: f32,
    range_end: f32,
    ratio: f64,
    pixel_count: u128,
}

impl OutputJson {
    pub fn new<C, F, T>(
        path: PathBuf,
        colorspace_name: &str,
        components_name: &str,
        counters: &[C],
        rgb_image: &RgbImage,
        filter: &F,
        _output_args: &OutputArgs,
        (_filtered_total_value, _filtered_total_pixel): (f64, u128),
    ) -> OutputJson
    where
        C: Counter,
        F: Filter<T>,
    {
        let tool_name = env!("CARGO_BIN_NAME").to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();

        let width = rgb_image.width();
        let height = rgb_image.height();
        let pixels = width as u128 * height as u128;

        let image = ImageData::new(path, width, height, pixels);

        let analysis = AnalysisData::new(
            colorspace_name,
            components_name,
            width,
            height,
            pixels,
            filter,
            counters,
        );

        OutputJson {
            tool_name,
            version,
            image,
            analysis,
        }
    }
}

impl ImageData {
    pub fn new(path: PathBuf, width: u32, height: u32, pixels: u128) -> ImageData {
        ImageData {
            path,
            width,
            height,
            pixels,
        }
    }
}

impl AnalysisData {
    pub fn new<C, F, T>(
        colorspace_name: &str,
        components_name: &str,
        width: u32,
        height: u32,
        total_pixel: u128,
        filter: &F,
        counters: &[C],
    ) -> AnalysisData
    where
        C: Counter,
        F: Filter<T>,
    {
        let colorspace = colorspace_name.to_string();
        let components = components_name.to_string();

        let hue_range = filter.hue_filter().map(|v| HueRange {
            start: v.start(),
            end: v.end(),
        });

        let bins: Vec<BinData> = counters
            .iter()
            .map(|c| BinData::new(c, total_pixel))
            .collect();

        AnalysisData {
            colorspace,
            components,
            hue_range,
            bins,
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

pub(crate) fn output<C, F, T>(
    colorspace_name: &str,
    components_name: &str,
    vec: &[C],
    rgb_image: &RgbImage,
    filter: &F,
    output_args: &OutputArgs,
    (filtered_total_value, filtered_total_pixel): (f64, u128),
) -> Result<(), ProcessError>
where
    C: Counter,
    F: Filter<T>,
{
    if !output_args.no_io {
        print_count(
            colorspace_name,
            components_name,
            vec,
            rgb_image.width(),
            rgb_image.height(),
            filter,
            filtered_total_value,
            filtered_total_pixel,
        );
    }

    Ok(())
}

fn print_count<C, F, T>(
    colorspace_name: &str,
    components_name: &str,
    vec: &[C],
    width: u32,
    height: u32,
    filter: &F,
    filtered_total_value: f64,
    filtered_total_pixel: u128,
) where
    C: Counter,
    F: Filter<T>,
{
    let total_pixel = ((width * height) as f32).max(1.0);

    println!("{} {}", colorspace_name, components_name);

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
