use std::env;
use std::process::exit;
use std::sync::mpsc::channel;

use notify::*;
use std::{path::Path, time::Duration};

use program::arguments::watch_source;

mod compiler;
mod modules;
mod optimizer;
mod pre_processor;
mod program;
mod tokens;

fn watch_files(paths: Vec<String>, args: &Vec<String>) -> Result<()> {
    // Create a channel to receive the events
    let (tx, rx) = channel();

    // Create a watcher with default config
    let mut watcher = RecommendedWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;

    // Watch each path
    for path in paths {
        watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;
    }

    // Handle incoming events
    for res in rx {
        match res {
            Ok(event) => {
                if let Some(_) = event.paths.first() {
                    let _ = request_build(true, &args.clone());
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}

fn request_build(watch: bool, args: &Vec<String>) -> Result<()> {
    let sourced_files: Vec<String> = compiler::compiler::build_source(args, &watch);

    if watch {
        watch_files(sourced_files.clone(), args)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} <command> [options]", args[0]);
        exit(1);
    }

    let command: &String = &args[1];

    if command == "build" || command == "b" {
        let should_watch: bool = watch_source(args.clone());
        let _ = request_build(should_watch, &args);
    } else if command == "mod" || command == "m" {
        if args.len() < 3 {
            println!("Usage: {} mod <module> [options]", args[0]);
            exit(1);
        }

        let module_name = &args[2];

        modules::modules::run_module(module_name.clone(), args.clone());
    } else {
        println!("Command {} not found", command);
        exit(1);
    }
}
