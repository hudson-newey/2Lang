use std::fs::File;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

pub fn delete_file(file_path: String) {
    if !std::path::Path::new(&file_path).exists() {
        return;
    }

    std::fs::remove_file(file_path)
        .expect("Something went wrong deleting the file");
}

pub fn write_binary(binary: Vec<u8>) {
    let mut full_binary: Vec<bool> = Vec::new();

    for bit in binary {
        full_binary.push(bit == 1);
    }

    println!();

    delete_file("a.out".to_string());
    write_file("a.out".to_string(), full_binary);

    make_file_executable("a.out".to_string());
}

fn write_file(file_path: String, binary: Vec<bool>) -> io::Result<()> {
    // Create a file for writing.
    let mut file = File::create(file_path)?;

    // Create a buffer to store the bits.
    let mut buffer: u8 = 0;
    let mut buffer_position = 0;

    // Iterate through the binary and pack them into bytes.
    for bit in binary {
        // Shift the current bits in the buffer to the left by one.
        buffer <<= 1;

        // Set the least significant bit to 1 if the bool is true.
        if bit {
            buffer |= 1;
        }

        // Increment the buffer position.
        buffer_position += 1;

        // If the buffer is full (8 bits), write it to the file and reset the buffer.
        if buffer_position == 8 {
            file.write_all(&[buffer])?;
            buffer = 0;
            buffer_position = 0;
        }
    }

    // If there are remaining bits in the buffer, write them to the file.
    if buffer_position > 0 {
        file.write_all(&[buffer])?;
    }

    Ok(())
}

fn make_file_executable(file_path: String) {
    let mut perms = std::fs::metadata(file_path.clone())
        .expect("Something went wrong getting the file permissions")
        .permissions();

    perms.set_mode(0o755);

    std::fs::set_permissions(file_path, perms)
        .expect("Something went wrong setting the file permissions");
}
