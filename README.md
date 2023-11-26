# Albert

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Albert](#albert)
  - [Installation](#installation)
    - [Ubuntu](#ubuntu)
    - [macOS](#macos)
    - [macOS (M1)](#macos-m1)
    - [Alpine](#alpine)
  - [Usage](#usage)
  - [Examples](#examples)
  - [Contributing](#contributing)
  - [License](#license)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

Perform mathematical operations on the command line.

**NOTE:** This is a work in progress and it is not ready for production use!
Expect any and all the APIs to change without notice prior to the first stable
release.

## Installation

All the distributed binaries have the corresponding checksums available. To
fetch the checksum, add the `.sha256` extension to the binary name. For
example, to fetch the checksum for the `al-x86_64-unknown-linux-gnu` binary,
run:

```bash
curl -LfO https://github.com/meysam81/albert/releases/latest/download/al-x86_64-unknown-linux-gnu
curl -LfO https://github.com/meysam81/albert/releases/latest/download/al-x86_64-unknown-linux-gnu.sha256
sha256sum -c al-x86_64-unknown-linux-gnu.sha256
```

### Ubuntu

```bash
curl -sSLf https://github.com/meysam81/albert/releases/latest/download/al-x86_64-unknown-linux-gnu -o al
chmod +x al
```

### macOS

```bash
curl -sSLf https://github.com/meysam81/albert/releases/latest/download/al-x86_64-apple-darwin -o al
chmod +x al
```

### macOS (M1)

```bash
curl -sSLf https://github.com/meysam81/albert/releases/latest/download/al-aarch64-apple-darwin -o al
chmod +x al
```

### Alpine

```bash
curl -sSLf https://github.com/meysam81/albert/releases/latest/download/al-x86_64-unknown-linux-musl  -o al
chmod +x al
```

## Usage

Currently the following mathematical operations are supported:

- Addition
- Subtraction
- Multiplication
- Division

## Examples

```bash
$ al 1 + 2
3
$ al 1 - 2 '*' 4
-7
$ al 1 - 2 '*' 4 / 2
-3
```

## Contributing

If you see any issue, have a feature request, or want to contribute, please
feel free to open an issue or a pull request. Any and all contributions are
welcome.

## License

This project is licensed under the [Apache License, Version 2.0](LICENSE).
