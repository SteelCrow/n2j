#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use chrono::Utc;
use clap::Parser;
use clio::{has_extension, ClioPath};
use error_stack::ResultExt;
use n2j::NmapRun;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    panic::Location,
};
use color_print::cformat;

type Result<T> = error_stack::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("file read error: unable to access or read the file")]
    ReadFile,

    #[error("file open error: unable to open input file")]
    OpenInputFile,

    #[error("file create error: unable to create output file")]
    CreateOutputFile,

    #[error("serialization error: failed to convert data to JSON format")]
    Serialization,

    #[error("XML parsing error: unable to parse the provided XML content")]
    Parsing,

    #[error("file write error: unable to access or write the file")]
    WriteFile,
}


/// A command-line tool to convert NMAP XML output to JSON format.
#[derive(Parser, Debug)]
#[command(version, 
            about, 
            long_about = None, 
            after_help = cformat!(
r#"<bold><underline>Examples:</underline></bold>

  # Convert single report file and pretty print to console 
   
    <bold>n2j nmap.xml --pretty</bold>

  # Convert all report files from directory and put them in another directory
  # <italic>/report/example.xml -> /reports/json/example.json</italic>
  # <italic>/report/example2.xml -> /reports/json/example2.json</italic>
  <bold>[!]</bold> output directory should exists 

    <bold>n2j /reports --output=/reports/json</bold>

  # Convert all report files from directory and put them into one file
  <bold>[!]</bold> output format ndjson by default

    <bold>n2j /reports --output=reports.json</bold>
"#))]
struct Args {
    /// A list of input files, directories, or stdin to parse.
    #[clap(value_parser, default_value = "-")]
    inputs: Vec<ClioPath>,

    /// Output JSON to file, directory or stdout.
    #[clap(long, short, value_parser, default_value = "-")]
    output: ClioPath,

    /// Additional context for errors
    #[clap(long, value_parser, default_value = "false")]
    debug: bool,

    /// Pretty format JSON output
    #[clap(long, value_parser, default_value = "false")]
    pretty: bool,
}

fn main() {
    error_stack::Report::install_debug_hook::<Location>(|_location, _context| {});

    let args = Args::parse();
    let debug = args.debug;

    match run(args) {
        Err(e) => {
            if debug {
                eprintln!("n2j: {e:#?}");
                std::process::exit(1);
            } else {
                eprintln!("n2j: {e}");
                std::process::exit(1);
            }
        }
        _ => {}
    }
}

fn run(args: Args) -> Result<()> {
    let mut output: Option<Box<dyn Write>> = if args.output.is_dir() {
        None
    } else {
        let output = args
            .output
            .clone()
            .create()
            .change_context(Error::CreateOutputFile)?;
        Some(Box::new(output))
    };

    for input in args.inputs {
        for file in input
            .files(has_extension("xml"))
            .change_context(Error::OpenInputFile)?
        {
            let mut file = file.open().change_context(Error::OpenInputFile)?;
            let json = read_content(&mut file, args.pretty)?;

            // Output is signle file or stdout
            if let Some(ref mut output) = output {
                output
                    .write_all(json.as_bytes())
                    .change_context(Error::WriteFile)?;
                output.write_all(b"\n").change_context(Error::WriteFile)?;
            // Output is dir
            } else if let Some(name) = file.path().file_name() {
                let base = args.output.path();
                // Input is stdin
                let path = if name == "-" {
                    base.join(format!(
                        "n2j-nmap-report-{}.json",
                        Utc::now().format("%Y-%m-%dT%H-%M-%S").to_string()
                    ))
                // Input is dir
                } else {
                    let mut path = base.join(name);
                    path.set_extension("json");
                    path
                };

                let mut file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path)
                    .change_context(Error::CreateOutputFile)?;
                file.write_all(json.as_bytes())
                    .change_context(Error::WriteFile)?;
                file.write_all(b"\n").change_context(Error::WriteFile)?;
            }
        }
    }

    Ok(())
}

fn read_content(mut reader: impl Read, pretty: bool) -> Result<String> {
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .change_context(Error::ReadFile)?;
    let report = NmapRun::parse_and_fix(content).change_context(Error::Parsing)?;

    if pretty {
        serde_json::to_string_pretty(&report)
    } else {
        serde_json::to_string(&report)
    }
    .change_context(Error::Serialization)
}
