use std::process::{exit, Command};
use std::{env, fs};

mod compiler;
mod modules;
mod optimizer;
mod pre_processor;
mod program;
mod tokens;

fn build(args: &Vec<String>) {
    let file_name: &String = if args.len() > 2 {
        &args[2]
    } else {
        println!("Usage: {} build <file_name> [options]", args[0]);
        exit(1)
    };

    let print_debug: bool = program::cla::log_debug(args.clone());
    if print_debug {
        println!("\nargs: {:?}", args);
        println!("file name: {}\n", file_name);
    }

    let output_file_path: String = program::cla::output_file_path(args.clone());

    let preprocessed_file_path: String = if program::cla::generate_intermediate(args.clone()) {
        let should_preserve_linked: bool = program::cla::preserve_linked(args.clone());
        let with_processor_comments: bool = program::cla::processor_comments(args.clone());

        pre_processor::pre_processor::pre_process(
            file_name.to_string(),
            &output_file_path,
            &should_preserve_linked,
            &with_processor_comments,
            print_debug,
        )
    } else {
        file_name.to_string()
    };

    if print_debug {
        println!("Linked file in: {}\n", preprocessed_file_path);
    }

    let compiled_file_path: Vec<u8> = compiler::read_file::read_file(preprocessed_file_path.clone());

    if print_debug {
        println!("Assembled file out: {}\n", output_file_path);
    }

    compiler::write_binary::write_binary(compiled_file_path, output_file_path.clone());

    if !program::cla::preserve_intermediate(args.clone()) {
        compiler::write_binary::delete_file(preprocessed_file_path.clone());
    }

    if program::cla::output_to_stdout(args.clone()) {
        let contents: String = fs::read_to_string(output_file_path.clone())
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

    if args.len() <= 1 {
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
