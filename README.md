# dupefiles

A fast and efficient command-line tool to find duplicate files in directories. Uses SHA256 hashing for reliable duplicate detection.

## Features

- Fast duplicate file detection using SHA256 hashing
- Filter by file extensions (e.g., *.jpg, *.pdf)
- CSV output format with human-readable file sizes
- Skips hidden files/directories and zero-byte files
- Safe handling of symlinks
- Performance timing output

## Installation

```bash
cargo build --release
```

## Command-Line Options

```
Usage: dupefiles [OPTIONS] <DIRECTORY>

Arguments:
  <DIRECTORY>  Directory to scan for duplicates

Options:
  -e, --extensions <EXTENSIONS>  Optional comma-separated list of file extensions to filter by (e.g., "mp4,jpg")
  -o, --output <FILE>           Optional output file path (if not specified, prints to stdout)
  -h, --help                    Print help
  -V, --version                 Print version
```

### Examples

1. Find all duplicates in a directory:
```bash
dupefiles ~/Downloads/
```

2. Find only duplicate images:
```bash
dupefiles -e "jpg,png,gif" ~/Downloads/
```

3. Save results to a file:
```bash
dupefiles -o results.csv ~/Downloads/
```

4. Combine extension filtering and output file:
```bash
dupefiles -e "pdf,doc,txt" -o documents.csv ~/Documents/
```

## Output Format

The tool outputs in CSV format with the following columns:
```
DUPE1.NAME,DUPE1.SIZE,DUPE1.HRSIZE,DUPE2.NAME,DUPE2.SIZE,DUPE2.HRSIZE
"/path/to/file1.jpg",85448,"83.4 KiB","/path/to/file2.jpg",85448,"83.4 KiB"
```

Where:
- `NAME`: Full path to the file
- `SIZE`: File size in bytes
- `HRSIZE`: Human-readable file size (e.g., "83.4 KiB")

## Notes

- Skips hidden files and directories (starting with '.')
- Skips zero-byte files
- Safely handles broken symlinks
- Shows elapsed time after completion
- Extensions can be specified with or without leading '.' or '*' (e.g., "*.jpg" and "jpg" are equivalent)
- Multiple extensions should be comma-separated without spaces (e.g., "jpg,png,pdf")
- When no output file is specified, results are printed to stdout in CSV format

## Development

Build with debug logging enabled:
```bash
cargo build --features debug
cargo run --features debug -- <args>
```

## License

[License information here]
