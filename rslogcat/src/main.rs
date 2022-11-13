use std::{fs::{self, File}, io::{self, BufRead, BufReader, Write}, ops::Range};

use colored::*;
use clap::Parser;
use anyhow::{Result, Ok};
use rayon::prelude::*;
use regex::Regex;
use itertools::Itertools;

mod cli;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    //println!("{:?}", args);
    //println!("{:?}", args.file);
    //println!("{:?}", args.pattern);
    //println!("{:?}", args.directories);

    let regex = Regex::new(args.pattern.as_str())?;

    args.directories.into_par_iter().for_each(|dir| {
        let entries = fs::read_dir(dir).unwrap()
                .map(|res| res.map(|e| e.path()))
                .filter(|entry| entry.as_ref().unwrap().is_file())
                .collect::<Result<Vec<_>, io::Error>>().unwrap();

        entries.into_par_iter().for_each(|v| {
            if let io::Result::Ok(file) = File::open(v.as_path()) {
                let reader = BufReader::new(file);
                let mut stdout = io::stdout();
                let matches: String = reader
                .lines()
                .enumerate()
                .map(|(lineno, line)| {
                    line.ok()
                        .map(|line| {
                            regex
                                .find(&line)
                                .map(|m| format_line(&line, lineno + 1, m.range()))
                        })
                        .flatten()
                })
                .filter_map(|v| v.ok_or(()).ok())
                .join("\n");

            if !matches.is_empty() {
                stdout.write_all(v.as_path().display().to_string().green().as_bytes()).unwrap();
                stdout.write_all(b"\n").unwrap();
                stdout.write_all(matches.as_bytes()).unwrap();
                stdout.write_all(b"\n").unwrap();
            }

            }
        });
    });

    Ok(())
}

pub fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];
    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        // 找到匹配项的起始位置，注意对汉字等非 ascii 字符，我们不能使用 prefix.len()
        // 这是一个 O(n) 的操作，会拖累效率，这里只是为了演示的效果
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}
