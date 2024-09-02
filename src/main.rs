use std::{fs::read_dir, io::Error, path::PathBuf, time::Instant};
use parser::parse::{parse, DateTime, PlayEntry};
use cli::run::{ run as run_cmd, Node };


pub mod parser;
pub mod cli;


fn get_paths(dir: &str) -> Result<Vec<PathBuf>, Error> {
    let r = read_dir(dir)?.filter(|x| x.is_ok()).map(|x| x.unwrap());
    let mut rv: Vec<PathBuf> = Vec::new();
    for path in r {
        rv.push(path.path())
    }
    Ok(rv)
}

fn main() {
    let mut table: Vec<PlayEntry> = Vec::new();
    let start_read_files = Instant::now();
    println!("getting file paths...");
    let paths = get_paths(r"X:\OneDrive\programmering\rs\spotify_data_explorer\data").expect("Could not get files");
    let elapsed_read_files = start_read_files.elapsed();
    println!("got file paths: {:.2?}", elapsed_read_files);

    println!("Parsing files...");
    let read_files_total = Instant::now();
    for path in paths {
        println!("  Reading {:?}", path.file_name().unwrap_or_default());
        let parse_time = Instant::now();

        let res = parse(path).unwrap();

        let elapsed_parse = parse_time.elapsed();
        println!("      - entries: {:.2?}", res.len());
        println!("      - time: {:.2?}", elapsed_parse);

        table.extend(res);

    }
    let elapsed_files_total = read_files_total.elapsed();
    println!("Parsed files: {:.2?}", elapsed_files_total);
    
    let db_node = Node::Table(table);
    let above_3s = Node::PlayTimeAbove(db_node.into(), 3000);
    let before = Node::During(above_3s.into(), DateTime { year: 2023, month: 11, day: 30, hour: 00, minute: 00 } , DateTime { year: 2023, month: 12, day: 01, hour: 02, minute: 00 });
    //let before = Node::After(above_3s.into(), DateTime { year: 2024, month: 07, day: 10, hour: 00, minute: 00 });
    let test_cmd = Node::Display(before.into());

    cli::run::run(test_cmd);

    println!("Hello, world!");
}
