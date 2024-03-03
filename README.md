# Rust WC Command Tool

This is a Rust implementation of the UNIX WC command tool.

## Installation

1. Install Rust from the [official website](https://www.rust-lang.org/).

## Download

Clone the repository:

git clone https://github.com/raihanmahmudiut/wc_tool.git


## Building the Project

Navigate to the project directory and run:

cargo build

## Running the Program

After building the project, you can run the WC command tool by executing the compiled binary file. Use the following command:


./target/debug/wc_tool



## Usage

Add the executable file `wc_tool.exe` to a directory that is included in the PATH variable. Go to Advanced System Settings to check the PATH. Now you can use the `wc_tool`.

The WC command tool supports the following options:

- `-b`: Count bytes
- `-l`: Count lines
- `-w`: Count words
- `-c`: Count characters

Example:

./target/debug/wc_tool -c text.txt



## Features

The `wc_tool` is a command-line utility written in Rust that provides functionality similar to the Unix `wc` command, which counts the number of lines, words, characters, and bytes in files or standard input. In addition to basic functionality, it offers the ability to count content specifically between the "Introduction" and "References" sections, making it particularly useful for analyzing academic writing.

### Features Summary:

1. **Counting Lines, Words, Characters, and Bytes**:
   - The tool can count the number of lines, words, characters, and bytes in a given text file or standard input.

2. **Flexible Input Handling**:
   - Accepts file paths as arguments to analyze specific files.
   - Can read from standard input if no file is provided as an argument.

3. **Selective Counting Between Introduction and References**:
   - Recognizes academic writing conventions by identifying content between the "Introduction" and "References" sections.
   - Counts only the content between these sections, helping researchers and writers analyze the main body of academic documents accurately.

## Example Usage

1. Count lines, words, characters, and bytes in a file:

wc_tool my_document.txt


2. Count only the number of words in a file:

wc_tool -w my_document.txt


3. Analyze content between the "Introduction" and "References" sections:

wc_tool -w my_academic_paper.txt


## Contributing

Contributions to the `wc_tool` are welcome! Feel free to submit issues or pull requests to improve its functionality or fix any bugs.

## License

This project is licensed under the [MIT License](LICENSE).