pub fn generic_error(message: String, line_number: usize, file_name: String) {
    println!("\nCompiler Error:");
    println!("\t{}:{}", file_name, line_number);
    println!("\t\t{:?}\n", message);
    std::process::exit(1);
}
