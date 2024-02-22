This is a version of UNIX WC command tool built using Rust as a part of Coding Challenges by John Crickett

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
Add the executable file wc_tool.exe to a directory that is included in the PATH variable. Go to Advanced System Settings to check the PATH.
Now you can use the wc_tool
The WC command tool supports the following options:

- `-b`: Count bytes
- `-l`: Count lines
- `-w`: Count words
- `-c`: Count characters

Example:

./target/debug/wc_tool -c filename.txt


## Contributing

Contributions are welcome! If you find any issues or want to add new features, please open an issue or pull request on GitHub.

## License

This project is licensed under the [MIT License](LICENSE).
