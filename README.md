# pxTally

[GitHub repository](https://github.com/tukinami/pxtally)

## これは何？

画像内のピクセルを色空間ごとに分類し、集計するCLIツールです。Rust製。

集計結果は以下のようになります。(`HSL`の`hue`について集計)

``` PowerShell
PS path\to\pxtally> pxtally.exe hsl hue --path C:\Users\Public\Pictures\something.png
  0.00 ->  30.00 :  14.26% (    149504 px)
 30.00 ->  60.00 :   4.59% (     48128 px)
 60.00 ->  90.00 :   4.30% (     45056 px)
 90.00 -> 120.00 :   5.18% (     54272 px)
120.00 -> 150.00 :  18.75% (    196608 px)
150.00 -> 180.00 :   3.81% (     39936 px)
180.00 -> 210.00 :   5.27% (     55296 px)
210.00 -> 240.00 :  16.02% (    167936 px)
240.00 -> 270.00 :   4.98% (     52224 px)
270.00 -> 300.00 :   6.84% (     71680 px)
300.00 -> 330.00 :   7.81% (     81920 px)
330.00 ->   0.00 :   8.20% (     86016 px)

 avr : 172.3714
```

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

## コマンドとオプション

それぞれの細かいオプションなどは、`pxtally <COMMAND> --help`や`pxtally <COMMAND> <SUBCOMMAND> --help`などを参照してください。

例:

``` PowerShell
PS path\to\pxtally> pxtally.exe --help
Usage: pxtally.exe <COMMAND>

Commands:
  hsl        Under HSL
  oklch      Under OKLCH
  img-oklch  Output the image processed under OKLCH
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `hsl`

``` PowerShell
PS path\to\pxtally> pxtally.exe hsl --help
Under HSL

Usage: pxtally.exe hsl <COMMAND>

Commands:
  hue             About hue
  saturation, -s  About saturation
  lightness, -l   About ligntness
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `oklch`

``` PowerShell
PS path\to\pxtally> pxtally.exe oklch --help
Under OKLCH

Usage: pxtally.exe oklch <COMMAND>

Commands:
  lightness, -l  About lightness
  chroma, -c     About chroma
  hue            About hue
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
  -l, --lightness <LIGHTNESS>  Number of lightness
  -c, --chroma <CHROMA>        Number of chroma
      --hue <HUE>              Number of hue
  -h, --help                   Print help
```


## 使用ライブラリ

いずれも敬称略。ありがとうございます。

+ [clap](https://github.com/clap-rs/clap) / Kevin B. Knapp and Clap Contributors
+ [image](https://github.com/image-rs/image) / The image-rs Developers
+ [color](https://github.com/linebender/color) / Raph Levien, Bruce Mitchener, Jr., Tom Churchman, Jordan Johnson

## ライセンス

MITにて配布いたします。

## 作成者

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)
