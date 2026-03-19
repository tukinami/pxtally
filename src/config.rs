use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::process;

#[derive(Parser, Debug)]
#[command(version, about = t!("help.about"), long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Analyze under HSL color space
    #[command(subcommand, about = t!("help.command.hsl"))]
    Hsl(HslCommands),
    /// Analyze under OKLCH color space
    #[command(subcommand,  about = t!("help.command.oklch"))]
    Oklch(OklchCommands),
    /// Analyze under OKLAB color space
    #[command(subcommand,  about = t!("help.command.oklab"))]
    Oklab(OklabCommands),
    /// Analyze under CIELCH color space
    #[command(subcommand,  about = t!("help.command.cielch"))]
    Cielch(CielchCommands),
    /// Analyze under CIELAB color space
    #[command(subcommand,  about = t!("help.command.cielab"))]
    Cielab(CielabCommands),
    /// Output the image processed under OKLCH
    #[command( about = t!("help.command.imgoklch"))]
    ImgOklch(ImgOklchArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum HslCommands {
    /// About hue
    #[command(short_flag = 'H', about = t!("help.subcommand.hue"))]
    Hue(AngleArgs),
    /// About saturation
    #[command(short_flag = 's', about = t!("help.subcommand.saturation", start = process::hsl::constants::SATURATION_MIN, end = process::hsl::constants::SATURATION_MAX))]
    Saturation(PercentageArgs),
    /// About lightness
    #[command(short_flag = 'l', about = t!("help.subcommand.lightness", start = process::hsl::constants::LIGHTNESS_MIN, end = process::hsl::constants::LIGHTNESS_MAX))]
    Lightness(PercentageArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum OklchCommands {
    /// About lightness
    #[command(short_flag = 'l', about = t!("help.subcommand.lightness", start = process::oklch::constants::LIGHTNESS_MIN, end = process::oklch::constants::LIGHTNESS_MAX))]
    Lightness(PercentageArgs),
    /// About chroma
    #[command(short_flag = 'c', about = t!("help.subcommand.chroma", start = process::oklch::constants::CHROMA_MIN, end = process::oklch::constants::CHROMA_MAX))]
    Chroma(ChromaArgs),
    /// About hue
    #[command(short_flag = 'H', about = t!("help.subcommand.hue"))]
    Hue(AngleArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum OklabCommands {
    /// About lightness
    #[command(short_flag = 'l', about = t!("help.subcommand.lightness", start = process::oklab::constants::LIGHTNESS_MIN, end = process::oklab::constants::LIGHTNESS_MAX))]
    Lightness(PercentageArgs),
    /// About a (green/red)
    #[command(short_flag = 'a', about = t!("help.subcommand.a", start = process::oklab::constants::A_MIN, end = process::oklab::constants::A_MAX))]
    A(PercentageArgs),
    /// About b (blue/yellow)
    #[command(short_flag = 'b', about = t!("help.subcommand.b", start = process::oklab::constants::B_MIN, end = process::oklab::constants::B_MAX))]
    B(PercentageArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum CielchCommands {
    /// About lightness
    #[command(short_flag = 'l', about = t!("help.subcommand.lightness", start = process::cielch::constants::LIGHTNESS_MIN, end = process::cielch::constants::LIGHTNESS_MAX))]
    Lightness(PercentageArgs),
    /// About chroma
    #[command(short_flag = 'c', about = t!("help.subcommand.chroma", start = process::cielch::constants::CHROMA_MIN, end = process::cielch::constants::CHROMA_MAX))]
    Chroma(ChromaArgs),
    /// About hue
    #[command(short_flag = 'H', about = t!("help.subcommand.hue"))]
    Hue(AngleArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum CielabCommands {
    /// About lightness
    #[command(short_flag = 'l', about = t!("help.subcommand.lightness", start = process::cielab::constants::LIGHTNESS_MIN, end = process::cielab::constants::LIGHTNESS_MAX))]
    Lightness(PercentageArgs),
    /// About a (green/red)
    #[command(short_flag = 'a', about = t!("help.subcommand.a", start = process::cielab::constants::A_MIN, end = process::cielab::constants::A_MAX))]
    A(PercentageArgs),
    /// About b (blue/yellow)
    #[command(short_flag = 'b', about = t!("help.subcommand.b", start = process::cielab::constants::B_MIN, end = process::cielab::constants::B_MAX))]
    B(PercentageArgs),
}

#[derive(Args, Debug)]
pub(crate) struct AngleArgs {
    /// Path to image
    #[arg(short, long, help = t!("help.args.common.path"))]
    pub path: PathBuf,

    /// Number of divisions for the range
    #[arg(short, long, default_value_t = 12, value_parser = clap::value_parser!(u16).range(1..=360), help = t!("help.args.common.divisor"))]
    pub divisor: u16,

    /// Start of the range
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u16).range(0..=360), help = t!("help.args.angle.start", start = 0, end = 360))]
    pub start: u16,

    /// Output method
    #[command(flatten)]
    pub output: OutputArgs,
}

#[derive(Args, Debug)]
pub(crate) struct PercentageArgs {
    /// Path to image
    #[arg(short, long, help = t!("help.args.common.path"))]
    pub path: PathBuf,

    /// Number of divisions for the range
    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u16).range(1..=100), help = t!("help.args.common.divisor"))]
    pub divisor: u16,

    /// Output method
    #[command(flatten)]
    pub output: OutputArgs,
}

#[derive(Args, Debug)]
pub(crate) struct ChromaArgs {
    /// Path to image
    #[arg(short, long, help = t!("help.args.common.path"))]
    pub path: PathBuf,

    /// Number of divisions for the range
    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u16).range(1..=100), help = t!("help.args.common.divisor"))]
    pub divisor: u16,

    /// Start of the hue range to extract
    #[arg(short, long, value_parser = oklch_hue_in_range, help = t!("help.args.chroma.starthue", start = 0, end = 360))]
    pub start_hue: Option<u16>,

    /// End of the hue range to extract
    #[arg(short, long, value_parser = oklch_hue_in_range, help = t!("help.args.chroma.endhue", start = 0, end = 360))]
    pub end_hue: Option<u16>,

    /// Output method
    #[command(flatten)]
    pub output: OutputArgs,
}

#[derive(Args, Debug)]
pub(crate) struct OutputArgs {
    /// Suppress formatted output to stdout
    #[arg(long, help = t!("help.args.output.noprint"))]
    pub no_print: bool,
    /// Output results as JSON to stdout
    #[arg(long, help = t!("help.args.output.json"))]
    pub json: bool,
    /// Write results as JSON to the specified file
    #[arg(long, value_name = "PATH", help = t!("help.args.output.jsonoutput"))]
    pub json_output: Option<PathBuf>,
    /// Force overwrite output file if it already exists
    #[arg(long, help = t!("help.args.common.force"))]
    pub force: bool,
}

#[derive(Args, Debug)]
pub(crate) struct ImgOklchArgs {
    /// Path to input image
    #[arg(short, long, help = t!("help.args.imgoklch.input"))]
    pub input: PathBuf,

    /// Path to output image
    #[arg(short, long, help = t!("help.args.imgoklch.output"))]
    pub output: PathBuf,

    /// Override value for lightness
    #[arg(short, long, value_parser = oklch_lightness_in_range, help = t!("help.args.imgoklch.lightness", start = process::oklch::constants::LIGHTNESS_MIN, end = process::oklch::constants::LIGHTNESS_MAX))]
    pub lightness: Option<f32>,

    /// Override value for chroma
    #[arg(short, long, value_parser = oklch_chroma_in_range, help = t!("help.args.imgoklch.chroma", start = process::oklch::constants::CHROMA_MIN, end = process::oklch::constants::CHROMA_MAX))]
    pub chroma: Option<f32>,

    /// Override value for hue
    #[arg(short = 'H', long, value_parser = oklch_hue_in_range, help = t!("help.args.imgoklch.hue", start = process::oklch::constants::HUE_MIN, end = process::oklch::constants::HUE_MAX))]
    pub hue: Option<u16>,

    /// Force overwrite output file if it already exists
    #[arg(long, help = t!("help.args.common.force"))]
    pub force: bool,
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
