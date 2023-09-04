use std::env;

mod binary_compiler {
    pub mod read_file;
    pub mod write_binary;
}

mod compiler {
    pub mod compile;
}

mod program {
    pub mod cla;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let binary_file_name: String =
        if program::cla::generate_intermediate(args.clone()) {
            compiler::compile::compile_file(file_name.to_string())
        } else {
            file_name.to_string()
        };

    let binary_value = binary_compiler::read_file::read_file(binary_file_name.clone());

    binary_compiler::write_binary::write_binary(binary_value);

    if !program::cla::preserve_intermediate(args) {
        binary_compiler::write_binary::delete_file(binary_file_name.clone());
    }
}
