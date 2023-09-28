# Cache/Temp Files Remover using Rust

## Description
This is a Rust program designed to remove cache and temporary files from a specified directory. It provides a simple and efficient way to clean up unnecessary files in a given directory.

## Usage
To use this program, follow these steps:

1. **Install Rust**: Before running the program, make sure you have Rust installed on your system. You can download and install Rust from [Rust's official website](https://www.rust-lang.org/).

2. **Compile the Program**:
   - Save the code you provided in a `.rs` file, e.g., `cache_cleaner.rs`.
   - Open your command prompt or terminal and navigate to the directory where you saved the file.
   - Compile the program by running the following command:
     ```shell
     rustc cache_cleaner.rs
     ```

3. **Run the Program**:
   - After successful compilation, you can run the program by executing the generated binary:
     ```shell
     ./cache_cleaner
     ```
     Note: On Windows, you might need to run it as `cache_cleaner.exe`.

4. **Cleaning Cache/Temp Files**:
   - The program will attempt to list all files in the directory specified in the `directory_path` variable (`C:\\Windows\\Temp` in this case).
   - It will then check if each entry is a file and delete it if it is.
   - Deleted files will be logged, and any errors encountered during file deletion will be displayed in the terminal.

## Author Contact
If you have any questions or need further assistance, you can reach out to the author:
- Author: Jenin Sutradhar
- Email: jeninsutradhar@gmail.com

Feel free to customize the program's `directory_path` variable to specify the directory you want to clean. Remember to exercise caution when using this program, especially in system directories, as it will permanently delete files.
