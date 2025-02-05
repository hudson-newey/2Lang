use std::env;
use std::process::exit;

use notify::*;
use std::{path::Path, time::Duration};

use program::arguments::watch_source;

mod compiler;
mod modules;
mod optimizer;
mod pre_processor;
mod program;
mod tokens;

fn request_build(args: &Vec<String>) {
    let sourced_file: &String = compiler::compiler::build_source(&args);

    let should_watch: bool = watch_source(args.clone());
    if should_watch {
        watch_file(&sourced_file, &args);
    }
}

fn watch_file(sourced_file: &String, args: &Vec<String>) {
    // we should never trigger this condition, but I don't know rust well enough
    // to know if this type of error would be caught by the compiler
    if sourced_file.len() < 1 {
        println!("Unable to watch: Source file count < 1");
        return;
    }

    let main_file: String = sourced_file.clone();

    // TODO: I probably want to use a debounced watcher
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        let config = Config::default().with_poll_interval(Duration::from_secs(1));
        Box::new(PollWatcher::new(tx, config).unwrap())
    } else {
        Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    };

    watcher
        .watch(Path::new(main_file.as_str()), RecursiveMode::Recursive)
        .unwrap();

    for _ in rx {
        request_build(&args);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} <command> [options]", args[0]);
        exit(1);
    }

    let command: &String = &args[1];

    if command == "build" || command == "b" {
        request_build(&args);
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
