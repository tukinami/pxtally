use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(subcommand)]
    /// Analyze under HSL color space
    Hsl(HslCommands),
    #[command(subcommand)]
    /// Analyze under OKLCH color space
    Oklch(OklchCommands),
    /// Output the image processed under OKLCH
    ImgOklch(ImgOklchArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum HslCommands {
    #[command(short_flag = 'H')]
    /// About hue
    Hue(AngleArgs),
    #[command(short_flag = 's')]
    /// About saturation
    Saturation(PercentageArgs),
    #[command(short_flag = 'l')]
    /// About lightness
    Lightness(PercentageArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum OklchCommands {
    #[command(short_flag = 'l')]
    /// About lightness
    Lightness(PercentageArgs),
    #[command(short_flag = 'c')]
    /// About chroma
    Chroma(ChromaArgs),
    #[command(short_flag = 'H')]
    /// About hue
    Hue(AngleArgs),
}

#[derive(Args, Debug)]
pub(crate) struct AngleArgs {
    /// Path to image
    #[arg(short, long)]
    pub path: PathBuf,

    /// Number of divisions for the range
    #[arg(short, long, default_value_t = 12, value_parser = clap::value_parser!(u16).range(1..=360))]
    pub divisor: u16,

    /// Start of the range
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u16).range(0..=360))]
    pub start: u16,

    /// Output method
    #[command(flatten)]
    pub output: OutputArgs,
}

#[derive(Args, Debug)]
pub(crate) struct PercentageArgs {
    /// Path to image
    #[arg(short, long)]
    pub path: PathBuf,

    /// Number of divisions for the range
    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u16).range(1..=100))]
    pub divisor: u16,

    /// Output method
    #[command(flatten)]
    pub output: OutputArgs,
}

#[derive(Args, Debug)]
pub(crate) struct ChromaArgs {
    /// Path to image
    #[arg(short, long)]
    pub path: PathBuf,

    /// Number of divisions for the range
    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u16).range(1..=100))]
    pub divisor: u16,

    /// Start of the hue range to extract
    #[arg(short, long, value_parser = oklch_hue_in_range)]
    pub start_hue: Option<u16>,

    /// End of the hue range to extract
    #[arg(short, long, value_parser = oklch_hue_in_range)]
    pub end_hue: Option<u16>,

    /// Output method
    #[command(flatten)]
    pub output: OutputArgs,
}

#[derive(Args, Debug)]
pub(crate) struct OutputArgs {
    /// Suppress formatted output to stdout
    #[arg(long)]
    pub no_print: bool,
    /// Output results as JSON to stdout
    #[arg(long)]
    pub json: bool,
    /// Write results as JSON to the specified file
    #[arg(long, value_name = "PATH")]
    pub json_output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub(crate) struct ImgOklchArgs {
    /// Path to input image
    #[arg(short, long)]
    pub input: PathBuf,

    /// Path to output image
    #[arg(short, long)]
    pub output: PathBuf,

    /// Override value for lightness
    #[arg(short, long, value_parser = oklch_lightness_in_range)]
    pub lightness: Option<f32>,

    /// Override value for chroma
    #[arg(short, long, value_parser = oklch_chroma_in_range)]
    pub chroma: Option<f32>,

    /// Override value for hue
    #[arg(short = 'H', long, value_parser = oklch_hue_in_range)]
    pub hue: Option<u16>,
}

fn oklch_hue_in_range(s: &str) -> Result<u16, String> {
    let value = s
        .parse::<u16>()
        .map_err(|_| format!("{s} is not a u16 number."))?;
    if (process::oklch::constants::HUE_MIN..process::oklch::constants::HUE_MAX)
        .contains(&(value as f32))
    {
        Ok(value)
    } else {
        Err(format!(
            "hue is not in range {}-{}",
            process::oklch::constants::HUE_MIN,
            process::oklch::constants::HUE_MAX
        ))
    }
}

fn oklch_lightness_in_range(s: &str) -> Result<f32, String> {
    float_in_range(
        s,
        process::oklch::constants::LIGHTNESS_MIN,
        process::oklch::constants::LIGHTNESS_MAX,
        "lightness",
    )
}

fn oklch_chroma_in_range(s: &str) -> Result<f32, String> {
    float_in_range(
        s,
        process::oklch::constants::CHROMA_MIN,
        process::oklch::constants::CHROMA_MAX,
        "chroma",
    )
}

fn float_in_range(s: &str, start: f32, end_include: f32, name: &str) -> Result<f32, String> {
    let value = s
        .parse::<f32>()
        .map_err(|_| format!("{s} is not a float number."))?;
    if (start..=end_include).contains(&value) {
        Ok(value)
    } else {
        Err(format!(
            "{} is not in range {}-{}",
            name, start, end_include
        ))
    }
}
