# Disk Usage CLI

A disk usage visualization tool with better output than `df -h`. Features colorful progress bars, smart filtering, and flexible sorting options.

## Features

- **Progress bars**: Visual bars showing disk usage percentage
- **Color coding**: Green for low usage, yellow for medium, red for high
- **Smart filtering**: Hide tiny partitions (< 1GB by default)
- **Flexible sorting**: Sort by usage, size, or mount point
- **Human-readable sizes**: Displays in GB/TB format

## Installation

### Build from source

```bash
cargo build --release
```

The binary will be available at `target/release/disk-usage`.

### Install globally

```bash
cargo install --path .
```

## Usage

### Basic usage

```bash
# Show disks >= 1GB, sorted by usage
disk-usage

# Show all disks including small ones
disk-usage --all

# Filter disks >= 10GB
disk-usage --min-size 10
```

### Sorting options

```bash
# Sort by usage percentage (default)
disk-usage --sort usage

# Sort by total size
disk-usage --sort size

# Sort by mount point
disk-usage --sort mount
```

### Visualization options

```bash
# Disable colors
disk-usage --no-color

# Disable progress bars
disk-usage --no-bars

# Both (plain text output)
disk-usage --no-color --no-bars
```

### Custom thresholds

```bash
# Set yellow threshold to 60% and red to 85%
disk-usage --yellow-threshold 60 --red-threshold 85
```

### Combine options

```bash
# Show all disks >= 5GB, sorted by size, with custom thresholds
disk-usage --min-size 5 --sort size --yellow-threshold 75 --red-threshold 90
```

## Example Output

```
Filesystem           Mounted on                     Size         Used         Available    Use%
/dev/nvme0n1p2       /                              456.89 GB    360.40 GB    96.49 GB     ████████████████░░░░ 78.9%
/dev/nvme1n1         /mnt/2tb_ssd                   1.79 TB      576.31 GB    1.23 TB      ██████░░░░░░░░░░░░░░ 31.4%

Total: 2.24 TB size, 936.71 GB used, 1.32 TB available
```

## Options

```
-s, --sort <FIELD>                Sort output by: usage, size, or mount [default: usage]
-m, --min-size <GB>               Minimum disk size to display in GB [default: 1]
-a, --all                         Show all disks (including those smaller than min-size)
    --no-color                    Disable colored output
    --no-bars                     Disable progress bars
    --yellow-threshold <PERCENT>  Set yellow threshold percentage [default: 70]
    --red-threshold <PERCENT>     Set red threshold percentage [default: 90]
-h, --help                        Print help
-V, --version                     Print version
```

## License

MIT
