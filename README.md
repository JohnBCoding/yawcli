# yawcli (Yet Another Weather CLI)

CLI tool that uses your IP address to provide local weather data. Does not work outside the USA as it uses the National Weather Service api for weather data.

## Usage

```
yawcli 0.2
Uses your IP to get the local forecast, only works in USA.

USAGE:
    yawcli.exe [FLAGS] [OPTIONS]

FLAGS:
    -c, --celsius    Converts temperature to celsius.
        --color      Prints out the forecast in color
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --hours <hours>    How many hours to show in hourly forecast, max 24. [default: 1]
```

## Install

If you currently have cargo installed you can simply run the following to install:

> run cargo install yawcli
