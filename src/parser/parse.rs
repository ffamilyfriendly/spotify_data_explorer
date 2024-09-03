use std::{
    cmp::Ordering,
    ffi::OsString,
    fmt::{self, Display, Formatter},
    fs, io,
    num::{IntErrorKind, ParseIntError},
    path::PathBuf,
};

#[derive(Copy, Clone, PartialEq)]
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
        f.write_str(&format!(
            "{:0>2}/{:0>2}/{:0>2} @ {:0>2}:{:0>2}",
            self.day, self.month, self.year, self.hour, self.minute
        ))
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

impl TryFrom<&str> for DateTime {
    type Error = DateTimeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
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

#[derive(Clone)]
pub struct PlayEntry {
    pub time: DateTime,
    pub artist: String,
    pub song: String,
    pub ms_played: u32,
    pub debug_info: DebugInfo,
}

#[derive(Default)]
struct Builder {
    buf: [String; 4],
    ptr: usize,
    pub vec: Vec<PlayEntry>,
}

impl Builder {
    pub fn append(&mut self, s: &str, d: DebugInfo) -> Result<(), DateTimeError> {
        if self.ptr == self.buf.len() - 1 {
            self.buf[self.ptr] = s.to_owned();
            self.ptr = 0;

            self.vec.push(PlayEntry {
                time: DateTime::try_from(self.buf[0].as_str())?,
                artist: self.buf[1].to_lowercase(),
                song: self.buf[2].to_lowercase(),
                ms_played: self.buf[3].parse()?,
                debug_info: d,
            });
        } else {
            self.buf[self.ptr] = s.to_owned();
            self.ptr += 1;
        }

        Ok(())
    }
}

pub fn parse(path: PathBuf) -> io::Result<Vec<PlayEntry>> {
    let mut fac = Builder::default();

    for (i, l) in fs::read_to_string(&path)?
        .lines()
        .filter(|l| l.contains(':'))
        .map(|l| l.trim())
        .enumerate()
    {
        let debug = DebugInfo {
            line: i,
            file_path: path.file_name().unwrap_or_default().to_owned(),
        };

        let sanitized = &l
            .split_at(l.find(':').unwrap_or_default() + 2)
            .1
            .replace('"', "");

        fac.append(sanitized, debug).expect("ERR");
    }

    Ok(fac.vec)
}
