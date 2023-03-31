# 正誤表 & FAQ

## 環境構築

以下のコマンドは実行する必要がありません (実行してもエラーになります) 。
手順を飛ばして、サンプルプロジェクトをダウンロードしてください。

```console
$ rustup target add riscv32imc-esp-espidf
```

## M5Stamp C3U Mate が USB デバイスとして認識されない

初回書き込み時のみ、以下のどちらかで認識するかどうか試してみてください。

- M5Stamp C3U Mate のボタンを押しながら USB ケーブルを本体に接続する
- USB ケーブルを接続した状態で、ボタンを押しながら、リセットボタンを押す

ボタンは M5Stamp C3U Mate の USB コネクタすぐ上にあります。

## ビルドに失敗する

以下のインストールを試してみてください。

```cosole
$ sudo apt install -y git curl gcc ninja-build cmake libudev-dev python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5
```

## 初回書き込み後、実行が止まってしまう

初回書き込み時のみ、下のログ出力後、実行が停止する場合があります。
いちど、USB ケーブルを抜き挿ししてみてください。

```
`Flashing has completed!
Commands:
CTRL+R Reset chip
CTRL+C Exit

ESP-ROM:esp32c3-api1-20210207
Build:Feb 7 2021
rst:0x15 (USB_UART_CHIP_RESET),boot:0x4 (DOWNLOAD(USB/UART0/1))
Saved PC:0x4004c97c
0x4004c97c - chip726_phyrom_version_num
at ??:??
waiting for download`
```
