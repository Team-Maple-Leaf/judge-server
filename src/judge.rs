use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::process::Command;

use tracing::info;

pub struct JudgeResult {
    pub status: Status,
    pub time: u64,
    pub memory: u64,
    pub message: String,
}

pub enum Status {
    Accepted,
    WrongAnswer,
    CompileError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError,
    SystemError,
}

impl JudgeResult {
    pub fn new(status: Status, time: u64, memory: u64, message: String) -> Self {
        Self {
            status,
            time,
            memory,
            message,
        }
    }
}

pub fn main(stasus: Status) {
    let input_files_path = "test_cases/input";
    let input_files = std::fs::read_dir(input_files_path).unwrap();
    let input_files_txt: Vec<_> = input_files
        .filter_map(|e| e.ok())
        .filter(|e| match e.path().extension() {
            Some(ext) => ext == "txt",
            None => false,
        })
        .collect();
    let input_len = input_files_txt.len();

    info!(?input_len, "input_len");

    let mut result = true;

    for i in 0..input_len {
        let output_path = String::from("test_cases/output/output") + &i.to_string() + ".txt";
        let answer_path = String::from("test_cases/result/result") + &i.to_string() + ".txt";

        let output_file = File::open(output_path).unwrap();
        let mut buf_reader = BufReader::new(output_file);
        let mut output_text = String::new();
        buf_reader.read_to_string(&mut output_text).unwrap();

        let answer_file = File::open(answer_path).unwrap();
        let mut buf_reader = BufReader::new(answer_file);
        let mut answer_text = String::new();
        buf_reader.read_to_string(&mut answer_text).unwrap();

        match output_text.trim_end() == answer_text.trim_end() {
            true => {
                println!("{}: Accepted", i);
            }
            false => {
                println!("{}: Wrong Answer", i);
                result = false;
            }
        }
    }

    Command::new("rm")
        .arg("a.out")
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on rm a.out");

    Command::new("sh")
        .arg("-c")
        .arg("rm ./test_cases/*/*.txt")
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on rm test_cases/output/*");

    Command::new("rm")
        .arg("main.c")
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on rm main.c");
}
