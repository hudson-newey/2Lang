use std::{env, fs};
use std::process::{exit, Command};

mod tokens;
mod compiler;
mod pre_processor;
mod program;
mod optimizer;
mod modules;

fn build(args: &Vec<String>) {
    let file_name: &String = if args.len() > 2 {
        &args[2]
    } else {
        println!("Usage: {} build <file_name> [options]", args[0]);
        exit(1)
    };

    if program::cla::log_debug(args.clone()) {
        println!("\nargs: {:?}", args);
        println!("file name: {}\n", file_name);
    }

    let input_file_name: String = if program::cla::generate_intermediate(args.clone()) {
        let should_expand_strings = !program::cla::no_expand_strings(args.clone());
        let should_preserve_linked = program::cla::preserve_linked(args.clone());

        pre_processor::pre_processor::pre_process(
            file_name.to_string(),
            &should_expand_strings,
            &should_preserve_linked
        )
    } else {
        file_name.to_string()
    };

    if program::cla::log_debug(args.clone()) {
        println!("File in: {}\n", input_file_name);
    }

    let compiled_file_path = compiler::read_file::read_file(input_file_name.clone());
    let output_file_path = program::cla::output_file_path(args.clone());

    if program::cla::log_debug(args.clone()) {
        println!("output file out: {}\n", output_file_path);
    }

    compiler::write_binary::write_binary(compiled_file_path, output_file_path.clone());

    if !program::cla::preserve_intermediate(args.clone()) {
        compiler::write_binary::delete_file(input_file_name.clone());
    }

    if program::cla::output_to_stdout(args.clone()) {
        let contents = fs::read_to_string(output_file_path.clone())
            .expect("Something went wrong reading the file");
        println!("{}", contents);
    }

    if program::cla::auto_run(args.clone()) {
        Command::new(output_file_path)
            .spawn()
            .expect("failed to execute process");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Usage: {} <command> [options]", args[0]);
        exit(1);
    }

    let command: &String = &args[1];

    if command == "build" {
        build(&args);
    } else if command == "mod" {
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
