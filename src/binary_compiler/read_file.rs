pub fn read_file(file_name: String) -> Vec<u8> {
    let mut binary_value = Vec::new();

    let contents =
        std::fs::read_to_string(file_name).expect("Something went wrong reading the file");

    for line in contents.lines() {
        for character in line.chars() {
            if character == '0' {
                binary_value.push(0);
            } else if character == '1' {
                binary_value.push(1);
            }
        }
    }

    return binary_value;
}
