use std::{
    cmp::Ordering, ffi::OsString, fmt::{self, Debug, Display, Formatter}, fs::File, io::{self, BufRead, BufReader}, num::{IntErrorKind, ParseIntError}, path::PathBuf, str::FromStr
};

use super::table::{Field, Table};

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct DateTime {
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub minute: u8,
    pub hour: u8,
}

impl DateTime {
    pub const fn unix_like(&self) -> u64 {
        let minute: u64 = 1000 * 60;
        let hour: u64 = minute * 60;
        let day: u64 = hour * 24;
        let month: u64 = day * 30;
        let year: u64 = day * 365;

        (self.year as u64 * year)
            + (self.month as u64 * month)
            + (self.day as u64 * day)
            + (self.hour as u64 * hour)
            + (self.minute as u64 * minute)
    }
}

impl Display for DateTime {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>2}/{:0>2}/{:0>2} @ {:0>2}:{:0>2}",
            self.day, self.month, self.year, self.hour, self.minute
        )
    }
}

impl Debug for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

#[derive(Debug)]
pub enum DateTimeError {
    ParseError(&'static str),
    ParseIntError(IntErrorKind),
}

impl From<ParseIntError> for DateTimeError {
    fn from(_value: ParseIntError) -> Self {
        DateTimeError::ParseIntError(_value.kind().to_owned())
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_unix = self.unix_like();
        let other_unix = other.unix_like();

        self_unix.partial_cmp(&other_unix)
    }
}

impl FromStr for DateTime {
    type Err = DateTimeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let sep = value.find(' ').ok_or(DateTimeError::ParseError(
            "found no date and time separator",
        ))?;
        let (date, time) = value.split_at(sep);
        let mut x = date.split('-');

        Ok(DateTime {
            year: x
                .next()
                .ok_or(DateTimeError::ParseError("unable to retrieve year"))?
                .parse()?,
            month: x
                .next()
                .ok_or(DateTimeError::ParseError("unable to retrieve month"))?
                .parse()?,
            day: x
                .next()
                .ok_or(DateTimeError::ParseError("unable to retrieve day"))?
                .parse()?,
            hour: time[1..3].parse()?,
            minute: time[4..6].replace(':', "").parse()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct DebugInfo {
    pub file_path: OsString,
    pub line: usize,
}


/* 
  {
    
    "username": "jonathanhermin",
    "platform": "Windows 7 (Unknown Ed) SP1 [x86 0]",
    "ms_played": 4991,
    "conn_country": "SE",
    "ip_addr_decrypted": "83.172.84.28",
    "user_agent_decrypted": null,
    "master_metadata_track_name": "Got To Be There",
    "master_metadata_album_artist_name": "Michael Jackson",
    "master_metadata_album_album_name": "The Very Best Of Michael Jackson With The Jackson 5",
    "spotify_track_uri": "spotify:track:5SId8ny8P11Ekz6KghvJYg",
    "episode_name": null,
    "episode_show_name": null,
    "spotify_episode_uri": null,
    "reason_start": "popup",
    "reason_end": "popup",
    "shuffle": false,
    "skipped": true,
    "offline": false,
    "offline_timestamp": 0,
    "incognito_mode": false
  },


*/

pub trait BuilderTrait {
    fn append(&mut self, s: &str, d: DebugInfo) -> Result<(), DateTimeError>;
}

pub struct SmallBuilder<'a> {
    buf: [String; 4],
    ptr: usize,
    pub table: &'a mut Table,
}

impl<'a> SmallBuilder<'a> {
    pub fn new(tbl: &'a mut Table) -> Self {
        SmallBuilder { buf: ["".to_owned(),"".to_owned(),"".to_owned(), "".to_owned()], ptr: 0, table: tbl }
    }
}

impl BuilderTrait for SmallBuilder<'_> {
    fn append(&mut self, s: &str, _d: DebugInfo) -> Result<(), DateTimeError> {
        if self.ptr == self.buf.len() - 1 {
            self.buf[self.ptr] = s.to_owned();
            self.ptr = 0;

            //println!("{:?}", self.buf);

            self.table.insert(
                [ 
                    Field::Date(self.buf[0].as_str().parse()?),
                    Field::String(self.buf[1].to_lowercase()),
                    Field::String(self.buf[2].to_lowercase()),
                    Field::Number(self.buf[3].parse().unwrap_or_default()),
                    ]).expect("COULD NOT INSERT");
        } else {
            self.buf[self.ptr] = s.to_owned();
            self.ptr += 1;
        }

        Ok(())
    }
}

pub struct BigBuilder<'a> {
    buf: [String; 21],
    ptr: usize,
    pub table: &'a mut Table,
}

impl<'a> BigBuilder<'a> {
    pub fn new(tbl: &'a mut Table) -> Self {
        let strarr: [String; 21] =  Default::default();
        BigBuilder { buf: strarr, ptr: 0, table: tbl }
    }
}

// "ts": "2012-02-08T13:00:55Z",

pub fn to_timestamp_big_history(value: &str) -> Result<DateTime, DateTimeError> {
    let sep = value.find('T').ok_or(DateTimeError::ParseError(
        "found no date and time separator",
    ))?;

    let (date, time) = value.split_at(sep);
    let mut date_segments = date.split('-');
    let mut time_segments = time.get(1..).unwrap().split(":");

    Ok(DateTime {
        year: date_segments
            .next()
            .ok_or(DateTimeError::ParseError("unable to retrieve year"))?
            .parse()?,
        month: date_segments
            .next()
            .ok_or(DateTimeError::ParseError("unable to retrieve month"))?
            .parse()?,
        day: date_segments
            .next()
            .ok_or(DateTimeError::ParseError("unable to retrieve day"))?
            .parse()?,
        hour: time_segments
            .next()
            .ok_or(DateTimeError::ParseError("unable to retrieve day"))?
            .parse()?,
        minute: time_segments
            .next()
            .ok_or(DateTimeError::ParseError("unable to retrieve day"))?
            .parse()?,
    })
}

impl BuilderTrait for BigBuilder<'_> {
    fn append(&mut self, s: &str, _d: DebugInfo) -> Result<(), DateTimeError> {
        if self.ptr == self.buf.len() - 1 {
            self.buf[self.ptr] = s.to_string();
            self.ptr = 0;

            //println!("{:?}", self.buf);

            self.table.insert(
                [ 
                    // "ts": "2010-11-02T15:42:08Z",
                    // - !!! - Field::Date(self.buf[0].parse()?),
                    Field::Date(to_timestamp_big_history(&self.buf[0])?),
                    // - !!! -
                    // "username": "jonathanhermin",
                    Field::String(self.buf[1].to_lowercase()),
                    // "platform": "Windows 7 (Enterprise Ed) SP0 [x86 0]",
                    Field::String(self.buf[2].to_lowercase()),
                    // "ms_played": 2890,
                    Field::Number(self.buf[3].parse().unwrap_or_default()),
                    // "conn_country": "SE",
                    Field::String(self.buf[4].to_lowercase()),
                    // "ip_addr_decrypted": "83.172.84.28",
                    Field::String(self.buf[5].to_lowercase()),
                    // "user_agent_decrypted": null,
                    Field::String(self.buf[6].to_lowercase()),
                    // "master_metadata_track_name": "The Black Lake",
                    Field::String(self.buf[7].to_lowercase()),
                    // "master_metadata_album_artist_name": "Patrick Doyle",
                    Field::String(self.buf[8].to_lowercase()),
                    // "master_metadata_album_album_name": "Harry Potter And The Goblet Of Fire (Original Motion Picture Soundtrack)",
                    Field::String(self.buf[9].to_lowercase()),
                    // "spotify_track_uri": "spotify:track:38GqVS4rDFnL0291QCiza9",
                    Field::String(self.buf[10].to_lowercase()),
                    // "episode_name": null,
                    Field::String(self.buf[11].to_lowercase()),
                    // "episode_show_name": null,
                    Field::String(self.buf[12].to_lowercase()),
                    // "spotify_episode_uri": null,
                    Field::String(self.buf[13].to_lowercase()),
                    // "reason_start": "clickrow",
                    Field::String(self.buf[14].to_lowercase()),
                    // "reason_end": "clickrow",
                    Field::String(self.buf[15].to_lowercase()),
                    // "shuffle": false, (BOOL !! !!)
                    Field::Bool(self.buf[16].to_lowercase() == "true"),
                    // "skipped": true,
                    Field::Bool(self.buf[17].to_lowercase() == "true"),
                    // "offline": false,
                    Field::Bool(self.buf[18].to_lowercase() == "true"),
                    // "offline_timestamp": 0,
                    Field::Number(self.buf[19].parse().unwrap_or_default()),
                    // "incognito_mode": null
                    Field::String(self.buf[20].to_lowercase()),
                    ]).expect("COULD NOT INSERT");
        } else {
            self.buf[self.ptr] = s.to_string();
            self.ptr += 1;
        }

        Ok(())
    }
}

pub fn parse(path: PathBuf, builder: &mut dyn BuilderTrait) -> io::Result<()> {

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut i = 0;
    for line in reader.lines() {
        i += 1;
        let l = line?.trim().to_owned();
        if !l.contains(":") { continue; }

        let debug = DebugInfo {
            line: i,
            file_path: path.file_name().unwrap_or_default().to_owned(),
        };

        let sanitized = &l
            .split_at(l.find(':').unwrap_or_default() + 2)
            .1
            .replace('"', "")
            .replace(',', "");

        builder.append(sanitized, debug).expect("ERR");
    }

    Ok(())
}
