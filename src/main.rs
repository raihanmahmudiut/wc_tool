use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn count_lines_words_chars_bytes<R: BufRead>(reader: R) -> (usize, usize, usize, usize) {
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut byte_count = 0;
    let mut after_introduction = false;

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains("References") {
                break;
            }

            if !after_introduction && line.contains("Introduction") {
                after_introduction = true;
            }

            if after_introduction {
                line_count += 1;
                let bytes = line.as_bytes().len();
                byte_count += bytes;
                word_count += line.split_whitespace().count();
                char_count += line.chars().count();
            }
        }
    }

    (line_count, word_count, char_count, byte_count)
}

fn count_stdin_lines_words_chars_bytes<R: BufRead>(reader: R) -> (usize, usize, usize, usize) {
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut byte_count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            line_count += 1;
            let bytes = line.as_bytes().len();
            byte_count += bytes;
            word_count += line.split_whitespace().count();
            char_count += line.chars().count();
        }
    }

    (line_count, word_count, char_count, byte_count)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let (lines, words, chars, bytes) = count_stdin_lines_words_chars_bytes(reader);
        println!("Bytes: {}", bytes);
        println!("Chars: {}", chars);
        println!("Words: {}", words);
        println!("Lines: {}", lines);
    } else if args.len() == 2 && !args[1].starts_with('-') {
        let file_path = &args[1];
        match File::open(file_path.as_str()) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let (lines, words, chars, bytes) = count_lines_words_chars_bytes(reader);
                println!("Bytes: {}", bytes);
                println!("Chars: {}", chars);
                println!("Words: {}", words);
                println!("Lines: {}", lines);
            }
            Err(_) => eprintln!("wc: {}: No such file or directory", file_path),
        }
    } else if args.len() >= 3 {
        let file_path = &args[2];
        match File::open(file_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let (lines, words, chars, bytes) = count_lines_words_chars_bytes(reader);

                match args[1].as_str() {
                    "-l" => println!("Lines: {}", lines),
                    "-w" => println!("Words: {}", words),
                    "-c" => println!("Chars: {}", chars),
                    "-b" => println!("Bytes: {}", bytes),
                    _ => println!("Usage: {} <option> <file>", args[0]),
                }
            }
            Err(_) => eprintln!("wc: {}: No such file or directory", file_path),
        }
    } else {
        println!("Usage: {} <option> <file>", args[0]);
    }
}
