<p align="center">
  <img src="./docs/ninja.svg" width="25%" alt="logo">
</p>
<p align="center">
  <em>Transform <b>nmap</b> XML reports into JSON</em>
</p>
<br />

## Overview

n2j is a library and CLI tool that helps you convert Nmap XML reports into JSON format.

## Getting Started

### Prerequisites

- Rust
- Cargo

### Installation

Install n2j using one of the following methods:

**Build from source:**

1. Clone this repo:

```sh
❯ git clone *this repo*
```

2. Build library:

```sh
❯ cargo build
```

3. Build cli-tool:

```sh
❯ cargo build --bin n2j --features=n2j_cli
```

> [!NOTE] 
> To quickly run cli-tool:
> ```sh
> cargo run --bin n2j --features=n2j_cli -- --help
> ```

## Usage

Example of library usage:

> [!NOTE] 
> There are only two important methods in this lib:
> ```rust
> impl NmapRun {
>    // Handles interrupted nmap scans with unclosed root tags. 
>    // Attempts to parse the XML, fixes the issue if detected, and retries parsing
>    pub fn parse_and_fix<'a>(xml: impl Into<Cow<'a, str>>) -> Result<Self>;
>    // Parses a valid Nmap XML report into a Rust struct
>    pub fn parse(xml: &str) -> Result<Self>;
> }
> ```

```rust
let path = "/path/to/your/report.xml"
let mut file = File::open(&path)?;
let mut content = String::new();
file.read_to_string(&mut content)?;

let report = NmapRun::parse_and_fix(content)?;

println!("{report:#?}");

// or if your want to get json
let report = serde_json::to_string_pretty(&report);

println!("{report}");
```

Example of cli-tool usage:

```sh
A command-line tool to convert NMAP XML output to JSON format

Usage: n2j [OPTIONS] [INPUTS]...

Arguments:
  [INPUTS]...  A list of input files, directories, or stdin to parse [default: -]

Options:
  -o, --output <OUTPUT>  Output JSON to file, directory or stdout [default: -]
      --debug            Additional context for errors
      --pretty           Pretty format JSON output
  -h, --help             Print help
  -V, --version          Print version

Examples:

  # Convert a single report file and pretty-print it to the console 
   
    n2j nmap.xml --pretty

  # Convert all report files from a directory save them in another directory
  # /report/example.xml -> /reports/json/example.json
  # /report/example2.xml -> /reports/json/example2.json
  [!] The output directory must exist

    n2j /reports --output=/reports/json

  # Convert all report files from a directory and combine them into one file
  [!] The default output format is NDJSON

    n2j /reports --output=reports.json
```

## Testing

> [!NOTE]
> To test if your reports are parsable, 
> place them in the reports folder and run:

Run the test suite using the following command:

```sh
❯ cargo test
```


## License

This project is protected under the [MIT License](https://github.com/surrealdb/license/blob/main/MIT.txt) License. For more details, refer to the [LICENSE](./LICENSE.md) file.