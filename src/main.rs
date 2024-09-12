use parser::{parse::{parse, BigBuilder, DateTime, SmallBuilder}, table::{Table, BIG_HISTORY_TABLE}};
use std::{fs::read_dir, io::Error, path::PathBuf, thread, time::Instant};

pub mod parser;

fn get_paths(dir: &str) -> Result<Vec<PathBuf>, Error> {
    Ok(read_dir(dir)?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .collect())
}

fn parse_file(file: PathBuf) -> (u8, Table) {

    let mut tbl = Table::new(BIG_HISTORY_TABLE);
    let mut builder = BigBuilder::new(&mut tbl);

    parse(file.clone(), &mut builder).unwrap();

    let filename = file.file_name().unwrap().to_str().unwrap();
    let parts: Vec<&str> = filename.split(&['_', '.'][..]).collect();
    let number =  parts.get(parts.len() - 2).unwrap();

    println!("{number}");

    (number.parse().unwrap(), tbl)
}

fn main() {
    let start_read_files = Instant::now();

    println!("getting file paths...");

    let paths = get_paths(r"X:\OneDrive\programmering\rs\spotify_data_explorer\data\full")
        .expect("Could not get files");
    let elapsed_read_files = start_read_files.elapsed();

    println!("got file paths: {elapsed_read_files:.2?}");

    println!("Parsing files...");

    let read_files_total = Instant::now();

    /*
    
      {
    "endTime" : "2023-08-27 22:44",
    "artistName" : "Vincent Neil Emerson",
    "trackName" : "Manhattan Island Serenade",
    "msPlayed" : 5150
  },
    
    */

    let mut tbl = Table::new(BIG_HISTORY_TABLE);

    let mut handles = vec![];

    for path in paths {

        let handle = thread::spawn(|| {
            parse_file(path)
        });

        handles.push(handle);
    }


    let mut res: Vec<(u8, Table)> = handles.into_iter().map(|t| t.join().expect("thread failed")).collect();
    res.sort_by(|a, b| a.0.cmp(&b.0));

    for table in res {
        tbl.rows.extend(table.1.rows);
    }

    let elapsed_files_total = read_files_total.elapsed();
    println!("Parsed files: {elapsed_files_total:.2?}");

    //tbl = tbl.field_is_greater_than("msplayed", &parser::table::Field::Number(109187.0)).unwrap();
    //tbl = tbl.field_is("artist", &parser::table::Field::String("lil darkie".to_owned())).unwrap();

    //println!("{}", tbl);
    println!("TOTAL LEN: {}", tbl.len());
}
