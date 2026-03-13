# pxTally

![Crates.io Version](https://img.shields.io/crates/v/pxtally)

A CLI tool that classifies and tallies pixels in an image by color space. Written in Rust.

It can be useful for analyzing the color tendencies of illustrations or photographs.

[日本語](https://github.com/tukinami/pxtally/blob/main/README.md) / English

[GitHub repository](https://github.com/tukinami/pxtally)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Tally (hsl / oklch)](#tally-hsl--oklch)
  - [Image Processing (img-oklch)](#image-processing-img-oklch)
  - [Help Option](#help-option)
- [Libraries](#libraries)
- [License](#license)
- [Author](#author)

## Features

- Classify pixels in an image by color space and output the tally results
- Output tally results as formatted text or JSON
- Modify pixel colors in an image in the `OKLCH` color space and output as a new image

Color spaces supported in the current version:

- `HSL`: (hue, saturation, lightness)
- `OKLCH`: (lightness, chroma, hue)

Supported image formats:

AVIF, BMP, DDS, EXR, FF, GIF, HDR, ICO, JPEG, PNG, PNM, QOI, TGA, TIFF, WebP

### Example Output

Tally result for `hue` under `HSL`:

``` PowerShell
PS path\to\pxtally> pxtally.exe hsl hue --path C:\Users\Public\Pictures\something.png
hsl hue
  0.00 ->  30.00 :   8.98% (     94208 px)
 30.00 ->  60.00 :   8.69% (     91136 px)
 60.00 ->  90.00 :   8.79% (     92160 px)
 90.00 -> 120.00 :   7.71% (     80896 px)
120.00 -> 150.00 :   7.71% (     80896 px)
150.00 -> 180.00 :   8.69% (     91136 px)
180.00 -> 210.00 :   8.79% (     92160 px)
210.00 -> 240.00 :   7.71% (     80896 px)
240.00 -> 270.00 :   7.71% (     80896 px)
270.00 -> 300.00 :   8.69% (     91136 px)
300.00 -> 330.00 :   8.79% (     92160 px)
330.00 ->   0.00 :   7.71% (     80896 px)

 avr : 177.7155
```

## Installation

### From the Releases Page

Download and extract the executable for your environment from the [latest release](https://github.com/tukinami/pxtally/releases/latest).

Note: Currently, only builds for `x86_64-pc-windows-msvc` and `i686-pc-windows-msvc` are provided.

You can run the executable directly from the extracted folder, or add it to your system `PATH` to call it from anywhere.

### Using cargo

pxTally is also available on [crates.io](https://crates.io/crates/pxtally).

If you have `cargo` installed, you can install it with the following command:

```
cargo install pxtally
```

## Usage

### Tally (hsl / oklch)

``` PowerShell
pxtally <COLOR_SPACE> <COMPONENT> --path <PATH>
```

Examples:

``` PowerShell
pxtally hsl saturation --path C:\Users\Public\Pictures\something.png
pxtally oklch chroma   --path C:\Users\Public\Pictures\something.png
```

#### Output Options

| Option | Description |
|---|---|
| (default) | Print formatted text to stdout |
| `--no-print` | Suppress formatted text output |
| `--json` | Print JSON to stdout (output even with `--no-print`) |
| `--json-output <PATH>` | Write JSON to a file |
| `--force` | Overwrite the file without confirmation |

The JSON schema is available in the [`schemas` directory](https://github.com/tukinami/pxtally/tree/main/schemas).

Examples:

``` PowerShell
# Print JSON only, suppressing formatted text
pxtally hsl saturation --path something.png --no-print --json

# Save as a JSON file
pxtally hsl saturation --path something.png --json-output ./result.json
```

### Image Processing (img-oklch)

Modifies the `lightness`, `chroma`, and/or `hue` of each pixel in an image under the `OKLCH` color space and outputs a new image.

``` PowerShell
pxtally img-oklch --input <INPUT_PATH> --output <OUTPUT_PATH> [OPTIONS]
```

Examples:

``` PowerShell
# Output with lightness fixed at 0.2
pxtally img-oklch --input something.png --output result.png --lightness 0.2
```

Note: If no parameters are specified, the output image will be nearly identical to the input.

You can also force overwriting existing files with the `--force` option.

Examples:

``` PowerShell
# Overwrite result.png without confirmation
pxtally img-oklch --input something.png --output result.png --lightness 0.2 --force
```

### Help Option

Detailed options for each command can be viewed with `--help`.

Examples:

``` PowerShell
pxtally --help
pxtally hsl --help
pxtally hsl hue --help
pxtally oklch --help
pxtally oklch chroma --help
pxtally img-oklch --help
```

## Libraries

Many thanks to the following (in no particular order):

+ [clap](https://github.com/clap-rs/clap) / Kevin B. Knapp and Clap Contributors
+ [image](https://github.com/image-rs/image) / The image-rs Developers
+ [color](https://github.com/linebender/color) / Raph Levien, Bruce Mitchener, Jr., Tom Churchman, Jordan Johnson
+ [serde](https://github.com/serde-rs/serde) / Erick Tryzelaar, David Tolnay
+ [serde_json](https://github.com/serde-rs/json) / Erick Tryzelaar, David Tolnay

## License

Distributed under the MIT license.

## Author

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)
