#[cfg(test)]
mod tests {
    #[test]
    fn test_pre_compiled() {
        let input_directory = "tests/input";
        let output_directory = "tests/output";

        let input_files = std::fs::read_dir(input_directory).unwrap();

        for file in input_files {
            let file = file.unwrap();
            let file_path = file.path();
            let file_path = file_path.to_str().unwrap();

            let file_name = file.file_name();
            let file_name = file_name.to_str().unwrap();

            // if the output file path does not exist, it means that the input
            // file should have thrown an error at compile time
            // and we should not expect there to be an output file
            let expected_output_path: String = format!("{}/{}.out", output_directory, file_name);
            let realized_output_path: String = format!("tmp/{}.test.out", file_name);
            let output_exists = std::path::Path::new(&expected_output_path).exists();

            let output = std::process::Command::new("cargo")
                .arg("run")
                .arg(file_path)
                .arg("-o")
                .arg(realized_output_path.clone())
                .output();

            if !output_exists {
                // assert_eq!(output.is_err(), true);
                continue;
            } else {
                let output_status = output.expect("Failed to run the compiler");
                assert_eq!(output_status.status.success(), true, "Failed for file: {}", file_name);

                let expected_output: Vec<u8> = std::fs::read(expected_output_path.clone()).unwrap();
                let realized_output: Vec<u8> = std::fs::read(realized_output_path.clone()).unwrap();

                assert_eq!(expected_output, realized_output, "Failed for file: {}", file_name);
            }
        }
    }
}
