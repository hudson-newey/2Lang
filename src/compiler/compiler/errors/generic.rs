pub fn generic_error(message: String, line_number: usize, file_name: String) {
    println!("\nCompiler Error:");
    println!("\t{}:{}\t\n{}\n", file_name, line_number, message);
    std::process::exit(1);
}
