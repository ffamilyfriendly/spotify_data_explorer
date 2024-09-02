use std::{ffi::OsString, fs::File, io::{self, Read}, num::{IntErrorKind, ParseIntError}, path::PathBuf};

#[derive(PartialEq)]
pub struct DateTime {
    pub day: u8,
    pub month: u8,
    pub year: u16,

    pub minute: u8,
    pub hour: u8
}

impl DateTime {
    pub fn unix_like(&self) -> u64 {
        let minute: u64 = 1000 * 60;
        let hour: u64 = minute * 60;
        let day: u64 = hour * 24;
        let month: u64 = day * 30;
        let year: u64 = day * 365;

        return ((self.year as u64 * year) + (self.month as u64 * month) + (self.day as u64 * day) + (self.hour as u64 * hour) + (self.minute as u64 * minute)).into()
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}/{}/{} @ {}:{}", self.day, self.month, self.year, self.hour, self.minute))?;
        
        Ok(())
    }
}

#[derive(Debug)]
pub enum DateTimeError {
    ParseError(String),
    ParseIntError(IntErrorKind)
}

impl From<ParseIntError> for DateTimeError {
    fn from(_value: ParseIntError) -> Self {
        DateTimeError::ParseIntError(_value.kind().to_owned())
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.unix_like() > other.unix_like() {
            Some(std::cmp::Ordering::Greater)
        } else if self.unix_like() < other.unix_like() {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl TryFrom<&str> for DateTime {
    type Error = DateTimeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sep = value.find(" ").ok_or(DateTimeError::ParseError("found no date and time separator".to_owned()))?;
        let (date, time) = value.split_at(sep);

        let hour = time.get(1..3).unwrap();
        let minutes = time.get(4..6).unwrap();

        let x: Vec<&str> = date.split("-").collect();

        Ok(DateTime {
            year: x[0].parse()?,
            month: x[1].parse()?,
            day: x[2].parse()?,
            hour: hour.parse()?,
            minute: minutes.replace(":", "").parse()?
        })

    }
}

#[derive(Debug)]
pub struct DebugInfo {
    pub file_path: OsString,
    pub line: usize,
}
pub struct PlayEntry {
    pub time: DateTime,
    pub artist: String,
    pub song: String,
    pub ms_played: u32,
    pub debug_info: DebugInfo
}

struct Builder {
    buf: [String; 4],
    ptr: usize,
    pub vec: Vec<PlayEntry>
}

impl Builder {
    pub fn new()-> Builder {
        Builder {
            buf: [String::new(), String::new(), String::new(), String::new()],
            ptr: 0,
            vec: Vec::new()
        }
    }

    pub fn append(&mut self, s: &str, d: DebugInfo) -> Result<(), DateTimeError> {
        if self.ptr == self.buf.len() - 1 {
            self.buf[self.ptr] = s.to_owned();
            self.ptr = 0;
            self.vec.push( PlayEntry {
                time: DateTime::try_from(self.buf[0].as_str())?,
                artist: self.buf[1].to_lowercase().to_owned(),
                song: self.buf[2].to_lowercase().to_owned(),
                ms_played: self.buf[3].parse()?,
                debug_info: d
            } )
        } else {
            self.buf[self.ptr] = s.to_owned();
            self.ptr += 1;
        }
        

        Ok(())
    }
}

pub fn parse(path: PathBuf) -> Result<Vec<PlayEntry>, io::Error> {
    let mut f = File::open(&path)?;
    let mut str_data = String::new();
    let _bytes = f.read_to_string(&mut str_data)?;
    
    let mut fac = Builder::new();

    let lines = str_data.lines();
    for (i, line) in lines.enumerate() {
        let l = line.trim();
        if l.contains(":") {
            let debug = DebugInfo {
                line: i,
                file_path: path.file_name().unwrap_or_default().to_owned()
            };

            let sanitized = &l.split_at(l.find(":").unwrap_or_default() + 2).1.replace('"', "");

            fac.append( &sanitized, debug ).expect("ERR");
        }
    }


    Ok(fac.vec)
}