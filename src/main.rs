use std::env;

mod compiler;
mod pre_processor;
mod program;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file_name> [options]", args[0]);
        return;
    }
    let file_name = &args[1];

    if program::cla::log_debug(args.clone()) {
        println!("\nargs: {:?}", args);
        println!("file name: {}\n", file_name);
    }

    let binary_file_name: String = if program::cla::generate_intermediate(args.clone()) {
        pre_processor::pre_processor::compile_file(file_name.to_string())
    } else {
        file_name.to_string()
    };

    if program::cla::log_debug(args.clone()) {
        println!("binary file in: {}\n", binary_file_name);
    }

    let binary_value = compiler::read_file::read_file(binary_file_name.clone());

    let output_file_path = program::cla::output_file_path(args.clone());

    if program::cla::log_debug(args.clone()) {
        println!("output file out: {}\n", output_file_path);
    }

    compiler::write_binary::write_binary(binary_value, output_file_path);

    if !program::cla::preserve_intermediate(args) {
        compiler::write_binary::delete_file(binary_file_name.clone());
    }
}
