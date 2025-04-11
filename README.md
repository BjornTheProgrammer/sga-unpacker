# SGA Unpacker
A simple cli tool to unpack .sga files from Relic.

## Installation
There are a few options of how to install sga-unpacker, below are listed the ways.

### Cargo

`cargo install --git https://github.com/BjornTheProgrammer/sga-unpacker`

### Binary

Click the releases tab, and then download and install the version you wish to use.

## Usage
Once installed, all you need to do is run sga-unpacker with an input file and specify an output dir. It then will be unpacked.

```
Usage: sga-unpacker --output <FILE> <INPUT>

Arguments:
  <INPUT>  Input file path

Options:
  -o, --output <FILE>  Output folder path
  -h, --help           Print help
  -V, --version        Print version
```

## Limitations
This has only been verified to work with AOE4 sga files, if you are experiencing any issues with other game sga files, just submit an issue, it shouldn't be too hard to implement it.

## Acknowledgement

Most of the code was translated from the C# project [`AOEMods.Essence`](https://github.com/aoemods/AOEMods.Essence). Even the documentation largely comes from there.
