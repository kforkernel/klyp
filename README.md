# klyp

A simple, lightweight CLI tool for gzip compression and decompression, built in Rust using `flate2`.

## Features

* **Encode & Decode:** Compress files to `.gz` and decompress them back to their original state.
* **Progress Bar:** Optional verbose mode (`-v`) displays a real-time progress bar using `indicatif`.
* **Minimalist:** Simple, straightforward command-line interface with no unnecessary bloat.

## Installation

### Prerequisites
* [Rust and Cargo](https://rust-lang.org) installed on your system.

### Build from Source

**Linux (Tested)**
```bash
git clone https://github.com/kforkernel/klyp
cd klyp
cargo build --release
```
The compiled binary will be located at `target/release/klyp`.

Move klyp into the `/usr/local/bin` directory:

```bash
sudo mv ./target/release/klyp /usr/local/bin
```

**macOS & Windows (Untested)**
Because this project uses standard Rust libraries (`std::fs`, `std::env`, `flate2`), it should compile and run natively on macOS and Windows, though it has not been officially tested on those platforms.

```bash
git clone https://github.com/kforkernel/klyp
cd klyp
cargo build --release
```

* **macOS binary path:** `target/release/klyp`
* **Windows binary path:** `target\release\klyp.exe`

## Usage

```bash
klyp [-e | -d] <SRC> <DST> [-v]
```

> [!NOTE]
> Note, `-v` flag must be at end of command.

### Flags & Arguments

| Argument / Flag | Description |
| :--- | :--- |
| `-e` | **Encode** (compress) the source file into gzip format. |
| `-d` | **Decode** (decompress) a gzip file. |
| `<SRC>` | Path to the input/source file. |
| `<DST>` | Path to the output/destination file. |
| `-v` | *(Optional)* **Verbose** mode. Displays a progress bar during processing. |

## Examples

### Compress a file
```bash
./target/release/klyp -e input.txt output.txt.gz
```
**Output:**
```text
Successfully processed 21 raw bytes.
Compression finished!
```

### Compress a file with progress bar (`-v`)
```bash
./target/release/klyp -e input.txt output.txt.gz -v
```
**Output:**
```text
⠉ [00:00:00] [########################################] 21 B/21 B (0s)
Successfully processed 21 raw bytes.
Compression finished!
```

### Decompress a file
```bash
./klyp -d output.txt.gz restored.txt
```

### Decompress a file with progress bar (`-v`)
```bash
./klyp -d output.txt.gz restored.txt -v
```

## Dependencies

* [`flate2`](https://crates.io/crates/flate2) – Gzip compression and decompression streams.
* [`indicatif`](https://crates.io/crates/indicatif) – Terminal progress bar for `-v` mode.

## License

This project is under the kforkernel Use License (KUL) v1.0.
For more details, read [LICENSE.md](LICENSE.md).
