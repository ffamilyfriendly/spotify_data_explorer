use parser::{parse::{parse, BigBuilder}, table::{Table, BIG_HISTORY_TABLE}, utils::quick_date};
use std::{fs::read_dir, io::Error, path::PathBuf, thread, time::{Duration, Instant}};

pub mod parser;

fn get_paths(dir: &str) -> Result<Vec<PathBuf>, Error> {
    Ok(read_dir(dir)?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .collect())
}

fn parse_file(file: PathBuf) -> (u8, Table, Duration) {
    let start_read_files = Instant::now();
    let mut tbl = Table::new(BIG_HISTORY_TABLE);
    let mut builder = BigBuilder::new(&mut tbl);

    parse(file.clone(), &mut builder).unwrap();

    let filename = file.file_name().unwrap().to_str().unwrap();
    let parts: Vec<&str> = filename.split(&['_', '.'][..]).collect();
    let number =  parts.get(parts.len() - 2).unwrap();

    println!("{number}");

    (number.parse().unwrap(), tbl, start_read_files.elapsed())
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


    let mut res: Vec<(u8, Table, Duration)> = handles.into_iter().map(|t| t.join().expect("thread failed")).collect();
    res.sort_by(|a, b| a.0.cmp(&b.0));

    for table in res {
        println!("[THREAD {}] parsing took {:.2?}", table.0, table.2);
        tbl.rows.extend(table.1.rows);
    }

    let elapsed_files_total = read_files_total.elapsed();
    println!("Parsed files: {elapsed_files_total:.2?}");


    let before_query = Instant::now();

    //tbl = tbl.field_in_range("time", &quick_date(2019, 12, 31).into(), &quick_date(2020, 01, 30).into()).unwrap();
    //tbl = tbl.sort_by("msplayed").unwrap();

    tbl = tbl.field_is_greater_than("msplayed", &3000.into()).unwrap();
    tbl = tbl.field_is("artist", &"lost dog street band".into()).unwrap();

    println!("QUERY TOOK: {:.2?}", before_query.elapsed());

    tbl = tbl.select([ "time", "song", "artist", "msplayed" ]);

    //println!("{}", tbl);
    println!("first row:\n{}", tbl.take_first().unwrap());

    //grouped = grouped.sort_by("COUNT").unwrap();

    
    //println!("unique tracks: {}", grouped);
}
