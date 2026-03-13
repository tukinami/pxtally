# pxTally

![Crates.io Version](https://img.shields.io/crates/v/pxtally)

画像内のピクセルを色空間ごとに分類し、集計するCLIツールです。Rust製。

イラストや写真の色傾向を把握したいときに活用できます。

日本語 / [English](https://github.com/tukinami/pxtally/blob/main/README.en.md)

[GitHub repository](https://github.com/tukinami/pxtally)

## 目次

- [できること](#できること)
- [インストール](#インストール)
- [使い方](#使い方)
  - [集計 (hsl / oklch)](#集計-hsl--oklch)
  - [画像加工 (img-oklch)](#画像加工-img-oklch)
  - [helpオプション](#helpオプション)
- [使用ライブラリ](#使用ライブラリ)
- [ライセンス](#ライセンス)
- [作成者](#作成者)

## できること

- 画像内のピクセルを色空間ごとに分類し、集計結果を出力する
- 集計結果を整形テキストまたはJSON形式で出力する
- `OKLCH`色空間で画像の色を変更し、新しい画像として出力する

現バージョンで対応している色空間:

- `HSL`: (hue, saturation, lightness)
- `OKLCH`: (lightness, chroma, hue)

対応している画像フォーマット:

AVIF, BMP, DDS, EXR, FF, GIF, HDR, ICO, JPEG, PNG, PNM, QOI, TGA, TIFF, WebP

### 出力例

`HSL`の`hue`について集計した結果:

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

## インストール

### リリースページから

[Releasesの最新版](https://github.com/tukinami/pxtally/releases/latest)から、使用環境にあった実行ファイルをダウンロード・展開してください。

注意: 現在、`x86_64-pc-windows-msvc` と `i686-pc-windows-msvc` 用のビルドのみ提供しています。

展開したフォルダをそのまま使うか、「パスを通す」ことで任意の場所から呼び出せます。

### cargo を使う場合

[crates.io](https://crates.io/crates/pxtally) にも登録しています。

`cargo` が使える環境であれば、以下のコマンドでインストールできます。

```
cargo install pxtally
```

## 使い方

### 集計 (hsl / oklch)

``` PowerShell
pxtally <COLOR_SPACE> <COMPONENT> --path <PATH>
```

例:

``` PowerShell
pxtally hsl saturation --path C:\Users\Public\Pictures\something.png
pxtally oklch chroma   --path C:\Users\Public\Pictures\something.png
```

#### 出力オプション

| オプション | 説明 |
|---|---|
| (デフォルト) | 整形されたテキストを標準出力に表示 |
| `--no-print` | 整形テキストの標準出力を抑制 |
| `--json` | JSON形式で標準出力に表示 (`--no-print` をつけていても出力されます) |
| `--json-output <PATH>` | JSON形式でファイルに出力 |
| `--force` | 強制的にファイルを上書き出力する |

JSONのスキーマは[`schemas`ディレクトリ](https://github.com/tukinami/pxtally/tree/main/schemas)にあります。

例:

``` PowerShell
# 整形テキストを抑制してJSONのみ標準出力
pxtally hsl saturation --path something.png --no-print --json

# JSONファイルとして保存
pxtally hsl saturation --path something.png --json-output ./result.json
```

### 画像加工 (img-oklch)

`OKLCH`色空間で画像の各ピクセルの`lightness`・`chroma`・`hue`を指定した値に変更し、新しい画像として出力します。

``` PowerShell
pxtally img-oklch --input <INPUT_PATH> --output <OUTPUT_PATH> [OPTIONS]
```

例:

``` PowerShell
# lightness を 0.2 に固定して出力
pxtally img-oklch --input something.png --output result.png --lightness 0.2
```

注意: 何も指定しなければ元の画像とほぼ変わらない画像が出力されます。

ファイルの上書き出力を強制する`--force`オプションもあります。

例:

``` PowerShell
# result.png が存在しても確認せずに上書き出力する
pxtally img-oklch --input something.png --output result.png --lightness 0.2 --force
```

### helpオプション

各コマンドの詳細オプションは `--help` で確認できます。

例:

``` PowerShell
pxtally --help
pxtally hsl --help
pxtally hsl hue --help
pxtally oklch --help
pxtally oklch chroma --help
pxtally img-oklch --help
```

## 使用ライブラリ

いずれも敬称略。ありがとうございます。

+ [clap](https://github.com/clap-rs/clap) / Kevin B. Knapp and Clap Contributors
+ [image](https://github.com/image-rs/image) / The image-rs Developers
+ [color](https://github.com/linebender/color) / Raph Levien, Bruce Mitchener, Jr., Tom Churchman, Jordan Johnson
+ [serde](https://github.com/serde-rs/serde) / Erick Tryzelaar, David Tolnay
+ [serde_json](https://github.com/serde-rs/json) / Erick Tryzelaar, David Tolnay

## ライセンス

MITにて配布いたします。

## 作成者

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)
