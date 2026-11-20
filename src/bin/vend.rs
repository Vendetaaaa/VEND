use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

use vend_ast::Abi;

struct Options {
    input: String,
    output: Option<String>,
    abi: Abi,
}

fn usage() -> &'static str {
    "Usage: vend <input.vend> [-o <output.s>] [--abi systemv|windows]\n\
     \n\
     Compile a VEND source file to Intel-syntax x86-64 assembly."
}

fn parse_options(args: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut args = args.skip(1);
    let mut input = None;
    let mut output = None;
    let mut abi = Abi::system_v();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Err(usage().to_string()),
            "-o" | "--output" => {
                output = Some(
                    args.next()
                        .ok_or_else(|| "missing path after --output".to_string())?,
                );
            }
            "--abi" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value after --abi".to_string())?;
                abi = match value.as_str() {
                    "systemv" => Abi::system_v(),
                    "windows" => Abi::windows_x64(),
                    _ => return Err(format!("unknown ABI '{}'; use systemv or windows", value)),
                };
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown option '{}'\n\n{}", value, usage()));
            }
            value if input.is_none() => input = Some(value.to_string()),
            value => return Err(format!("unexpected argument '{}'\n\n{}", value, usage())),
        }
    }

    let input = input.ok_or_else(|| usage().to_string())?;
    Ok(Options { input, output, abi })
}

fn compile(options: Options) -> Result<String, String> {
    let source = fs::read_to_string(&options.input)
        .map_err(|error| format!("cannot read '{}': {}", options.input, error))?;
    vend_ast::compile(&source, options.abi).map_err(|diagnostics| {
        diagnostics
            .iter()
            .map(|diagnostic| diagnostic.format_with_source(&source))
            .collect::<Vec<_>>()
            .join("\n")
    })
}

fn run() -> Result<(), String> {
    let options = parse_options(env::args())?;
    let output_path = options.output.clone();
    let assembly = compile(options)?;

    if let Some(path) = output_path {
        fs::write(&path, assembly)
            .map_err(|error| format!("cannot write '{}': {}", path, error))?;
    } else {
        io::stdout()
            .write_all(assembly.as_bytes())
            .map_err(|error| format!("cannot write assembly to stdout: {}", error))?;
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if error == usage() {
                eprintln!("{}", error);
            } else {
                eprintln!("vend: {}", error);
            }
            ExitCode::from(1)
        }
    }
}
