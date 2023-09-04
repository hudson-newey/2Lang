use std::env;

mod binary_compiler {
    pub mod read_file;
    pub mod write_binary;
}

mod compiler {
    pub mod compile;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let binary_file_name: String = compiler::compile::compile_file(file_name.to_string());

    let binary_value = binary_compiler::read_file::read_file(binary_file_name);

    binary_compiler::write_binary::write_binary(binary_value);
}
