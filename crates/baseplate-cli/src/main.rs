//! The `baseplate` command.

use std::env;
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: &str = "\
baseplate: describe it, Baseplate builds it in Roblox Studio

Usage:
  baseplate                      Start a session in this folder
  baseplate \"<request>\"          Run one turn, then exit
  baseplate plugin install       Copy the Baseplate plugin into Studio's Plugins folder
  baseplate doctor               Check Studio, the MCP server and the plugin

Options:
  --model <name>                 ollama/<model> (default, free and local), or with your
                                 own key: claude-sonnet-5-5, gpt-5, gemini-2.5-pro
  --mode <ask|auto|plan>         Ask before changes (default), apply on your own, or plan only
  --studio <id>                  Pick a Studio window when more than one is open
  -V, --version                  Print the version
  -h, --help                     Print this help

In a session:
  /plan  /diff  /playtest  /rewind [turn]  /model  /rules  /publish  /help
";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("-V" | "--version") => {
            println!("baseplate {VERSION}");
            ExitCode::SUCCESS
        }
        Some("-h" | "--help") | None => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("baseplate: unknown command '{other}'. Run `baseplate --help`.");
            ExitCode::from(2)
        }
    }
}
