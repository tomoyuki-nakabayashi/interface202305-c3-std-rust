# http-app

Final application.

## Prerequisites

1. Please edit `cfg.toml` according to your Wi-Fi environment both for server and client.

## Build & Run

```console
# at server directory
$ cargo espflash --release --monitor /dev/SERIAL_DEVICE
```

```console
# at client directory
$ cargo espflash --release --monitor /dev/SERIAL_DEVICE
```
