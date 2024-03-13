# count-hours
CLI tool to help manage hours worked on a project.

### Documentation quick links

* [Installation](#installation)
* [User Guide](#usage)
* [File format](#file-format)
* [Make Release](RELEASE-CHECKLIST.md)

## Installation
### Download
Precompiled binaries can be downloaded [here](https://github.com/hoel-bagard/count-hours/releases).

### Building
This program is written in Rust, so you will need to [install Rust](https://www.rust-lang.org/) in order to compile it.

```console
git clone https://github.com/hoel-bagard/count-hours.git
cd count-hours
cargo build --release
```

The resulting binary will be in `./target/release/count-hours`. You can check that the binary works with:

```console
./target/release/count-hours --version
```

### Add binary to path
For convenience, you can place the binary in a folder in the PATH. For example:

```console
sudo cp ./target/release/count-hours /usr/local/bin
```

or

```bash
cp ./target/release/count-hours ~/.local/bin
```

If using `~/.local/bin`, you might need to add it to the PATH. For this, you can place this in your `.rc` or `.profile` file:
```bash
if ! [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
  PATH=$PATH:$HOME/.local/bin
fi
```

## Usage
### Log mode
The log mode is used to simply add the current timestamp to a file:

```console
count-hours log <start/end> <file-path>
```

### Report mode
The report mode processes the file created with the log command to make it easier to copy/paste into an excel sheet.

#### Get total amount of hours worked:
```console
count-hours report total <file-path> -t <target month> -w <hourly wage>
```

For example:

```console
count-hours report total ~/work/my-project/.count-hours.csv -t 12 -w 3000
```

(The target month and hourly wage are optional.)

#### Get the a list of the start/end timestamps
```console
count-hours report starts <file-path>
```

For example:

```console
count-hours report starts ~/work/my-project/.count-hours.csv
```

## File format
The data is store in a simple CSV format and can therefore be edited by hand if required. The timestamps use the [`%Y-%m-%d %H:%M:%S` format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

```code
<start time 1>,<end time 1>
<start time 2>,<end time 2>
```

For example
```code
2023-12-29 20:30:00,2023-12-29 23:30:00
2023-12-30 19:00:00,2023-12-30 22:00:00
```

## TODO:
- Make Windows binary using [cross](https://github.com/cross-rs/cross) ?  (see [here](https://github.com/BurntSushi/ripgrep/blob/master/.github/workflows/release.yml) for an example)
