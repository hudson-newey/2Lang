use std::env;

mod binary_compiler {
    pub mod read_file;
    pub mod write_binary;
}

mod compiler {
    pub mod compiler;
}

mod program {
    pub mod cla;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    if program::cla::log_debug(args.clone()) {
        println!("\nargs: {:?}", args);
        println!("file name: {}\n", file_name);
    }

    let binary_file_name: String = if program::cla::generate_intermediate(args.clone()) {
        compiler::compiler::compile_file(file_name.to_string())
    } else {
        file_name.to_string()
    };

    if program::cla::log_debug(args.clone()) {
        println!("binary file in: {}\n", binary_file_name);
    }

    let binary_value = binary_compiler::read_file::read_file(binary_file_name.clone());

    let output_file_path = program::cla::output_file_path(args.clone());

    if program::cla::log_debug(args.clone()) {
        println!("output file out: {}\n", output_file_path);
    }

    binary_compiler::write_binary::write_binary(binary_value, output_file_path);

    if !program::cla::preserve_intermediate(args) {
        binary_compiler::write_binary::delete_file(binary_file_name.clone());
    }
}
