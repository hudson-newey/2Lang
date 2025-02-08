use std::process::{exit, Command};
use std::fs;

use crate::pre_processor::pre_processor::PreProcessorOutput;
use crate::program;
use crate::pre_processor;
use crate::compiler;

/* returns: an array of files that was processed as part of the build */
pub fn build_source(args: &Vec<String>) -> Vec<String> {
    let file_name: &String = if args.len() > 2 {
        &args[2]
    } else {
        println!("Usage: {} build <file_name> [options]", args[0]);
        exit(1)
    };

    let mut source_files: Vec<String> = Vec::new();
    source_files.push(file_name.clone());

    let print_debug: bool = program::arguments::log_debug(args.clone());
    if print_debug {
        println!("\nargs: {:?}", args);
        println!("file name: {}\n", file_name);
    }

    let output_file_path: String = program::arguments::output_file_path(args.clone());

    // test if there is a directory for the output file, if it doesn't exist
    // we want to create a directory for it

    let pre_processor_output: PreProcessorOutput = if program::arguments::generate_intermediate(args.clone()) {
        let should_preserve_linked: bool = program::arguments::preserve_linked(args.clone());
        let with_processor_comments: bool = program::arguments::processor_comments(args.clone());

        pre_processor::pre_processor::pre_process(
            file_name.to_string(),
            &output_file_path,
            &should_preserve_linked,
            &with_processor_comments,
            print_debug,
        )
    } else {
        PreProcessorOutput {
            output_path: file_name.to_string(),
            source_files: vec![],
        }
    };

    let preprocessed_file_path: String = pre_processor_output.output_path;
    for source_file in pre_processor_output.source_files {
        source_files.push(source_file.clone());
    }

    if print_debug {
        println!("Linked file in: {}\n", preprocessed_file_path);
    }

    let compiled_file_path: Vec<u8> = compiler::read_file::read_file(preprocessed_file_path.clone());

    if print_debug {
        println!("Assembled file out: {}\n", output_file_path);
    }

    compiler::write_binary::write_binary(compiled_file_path, output_file_path.clone());

    if !program::arguments::preserve_intermediate(args.clone()) {
        compiler::write_binary::delete_file(preprocessed_file_path.clone());
    }

    if program::arguments::output_to_stdout(args.clone()) {
        let contents: String = fs::read_to_string(output_file_path.clone())
            .expect("Something went wrong reading the file");
        println!("{}", contents);
    }

    if program::arguments::auto_run(args.clone()) {
        Command::new(output_file_path)
            .spawn()
            .expect("failed to execute process");
    }

    return source_files.clone();
}

