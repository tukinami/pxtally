# pxTally

[GitHub repository](https://github.com/tukinami/pxtally)

## これは何？

画像内のピクセルを色空間ごとに分類し、集計するCLIツールです。Rust製。

集計結果は以下のようになります。(`HSL`の`hue`について集計)

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

集計結果は上記のような整形された標準出力の他、JSON形式でも出力できます。
JSONのスキーマは[`schemas`ディレクトリ](https://github.com/tukinami/pxtally/tree/main/schemas)にあります。

現在対応している色空間は以下。

- `HSL`: (hue, saturation, lightness)
- `OKLCH`: (lightness, chroma, hue)

余力があれば追加します。

また、追加の機能として、`OKLCH`に変換した後、用途によって色を変更した画像を出力する機能があります。

## 使い方

ダウンロードは[Releasesの最新版](https://github.com/tukinami/pxtally/releases/latest)から使用環境にあった実行ファイルをダウンロード・展開してください。

(※現在、`x86_64-pc-windows-msvc`と`i686-pc-windows-msvc`用のビルドしかありません)

実行ファイルのある場所を、仮に`path/to/pxtally`とします。

コマンドプロンプト、PowerShellなどのシェルで、`path/to/pxtally`に移動します。

その後、Windowsの場合、`pxtally.exe --help`入力し、決定します。(他のOSの場合は実行ファイルの名前を適宜読み替えてください)

使い方が出てくるので、それに従って使用してください。

なお、「パスを通す」の意味が分かる方は、そちらの方法でも大丈夫です。

## 基本的なコマンドとオプション

### `img-oklch`以外

`pxtally <colorspace> <component> --path <PATH>`のように使用します。

例:

+ `pxtally hsl saturation --path C:\Users\Public\Pictures\something.png`
+ `pxtally oklch chroma --path C:\Users\Public\Pictures\something.png`

整形されたものがデフォルトで標準出力されます。
これを抑えたいときは、`--no-print`のオプションを付けてください。

例: `pxtally hsl saturation --path C:\Users\Public\Pictures\something.png --no-print`

JSON形式でも出力できます。標準出力に出力したいときは`--json`オプションを付けます。`--no-print`をつけていても出力されます。

例: `pxtally hsl saturation --path C:\Users\Public\Pictures\something.png --json`

JSON形式のファイルで出力したい場合は、`--json-output <PATH>`を入力してください。

例: `pxtally hsl saturation --path C:\Users\Public\Pictures\something.png --json-output ./test.json`

### `img-oklch`

`pxtally img-oklch --input <INPUT_PATH> --output <OUTPUT_PATH>`のように使用します。

例: `pxtally img-oklch --input C:\Users\Public\Pictures\something.png --output ./test.png`

このままでは大して変わらない画像が出力されると思います。

このコマンドでは、`lightness`、`chroma`、`hue`の値を一定に変更できます。
例えば、`lightness`を`0.2`にするときは、以下のようになります。

例: `pxtally img-oklch --input C:\Users\Public\Pictures\something.png --output ./test.png --lightness 0.2`

## 色空間ごとのヘルプ出力

それぞれの細かいオプションなどは、`pxtally <COMMAND> --help`や`pxtally <COMMAND> <SUBCOMMAND> --help`などを参照してください。

例:

``` PowerShell
PS path\to\pxtally> pxtally.exe --help
CLI tool to tally pixels.

Usage: pxtally.exe <COMMAND>

Commands:
  hsl        Analyze under HSL color space
  oklch      Analyze under OKLCH color space
  img-oklch  Output the image processed under OKLCH
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `hsl`

``` PowerShell
PS path\to\pxtally> pxtally.exe hsl --help
Analyze under HSL color space

Usage: pxtally.exe hsl <COMMAND>

Commands:
  hue, -H         About hue
  saturation, -s  About saturation
  lightness, -l   About lightness
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `oklch`

``` PowerShell
PS path\to\pxtally> pxtally.exe oklch --help
Analyze under OKLCH color space

Usage: pxtally.exe oklch <COMMAND>

Commands:
  lightness, -l  About lightness
  chroma, -c     About chroma
  hue, -H        About hue
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `img-oklch`

``` PowerShell
PS path\to\pxtally> pxtally.exe img-oklch --help
Output the image processed under OKLCH

Usage: pxtally.exe img-oklch [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>          Path to input image
  -o, --output <OUTPUT>        Path to output image
  -l, --lightness <LIGHTNESS>  Override value for lightness
  -c, --chroma <CHROMA>        Override value for chroma
  -H, --hue <HUE>              Override value for hue
  -h, --help                   Print help
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
