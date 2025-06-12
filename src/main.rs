#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod cli;
mod flux;
mod gui;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    // println!("{:?}", args);
    if !args.dry_run {
        let res = match args.command {
            Some(cli::Commands::Gui) => gui::start(),
            // Some(cli::Commands::Cli) => flux::create(args),
            None => flux::create(args),
        };

        match res {
            Ok(s) => {
                println!("{}", s)
            }
            Err(e) => {
                eprintln!("error: {}", e)
            }
        }
    }
}
