use std::fmt::{Debug, Display};

use super::parse::DateTime;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Field {
    Date(DateTime),
    String(String),
    Number(f64)
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Field::String(s) => s,
            Field::Number(n) => &n.to_string(),
            Field::Date(d) => &format!("{}", d)
        };

        f.write_str(as_str)
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Field::String(value)
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        Field::String(value.to_owned())
    }
}

#[derive(Clone)]
pub struct Row {
    pub fields: Vec<Field>
}


/*

                    // "ts": "2010-11-02T15:42:08Z",
                    Field::Date(self.buf[0].parse()?),
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

                    Field::String(self.buf[16].to_lowercase()),
                    // "skipped": true,

                    Field::String(self.buf[17].to_lowercase()),
                    // "offline": false,

                    Field::String(self.buf[18].to_lowercase()),
                    // "offline_timestamp": 0,

                    Field::Number(self.buf[19].parse().unwrap_or_default()),
                    // "incognito_mode": null
                    Field::String(self.buf[20].to_lowercase()),

*/

pub static BIG_HISTORY_TABLE: [&str; 21] = ["ts", "username", "platform", "msplayed", "country", "ip_addr", "user_agent", "song", "artist", "album", "track_uri", "episode_name", "episode_show_name", "episode_uri", "reason_start", "reason_end", "shuffle", "skipped", "offline", "offline_timestamp", "incognito_mode"];

pub struct Table {
    pub header: Vec<(String, usize)>,
    pub rows: Vec<Row>
}

pub enum DataErrors {
    NotFound(String),
    TooManyValues
}

impl Debug for DataErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(s) => f.write_str(s),
            Self::TooManyValues => f.write_str("got too many values")
        }
    }
}

impl Table {
    pub fn new<const T: usize>(header: [&str; T]) -> Self {

        let mut v: Vec<(String,usize)> = Vec::new();

        for i in 0..header.len() {
            v.push((header[i].to_string(), i));
        }

        Table {
            header: Vec::from(v),
            rows: Vec::new()
        }
    }

    pub fn insert<const T: usize>(&mut self, row: [Field; T]) -> Result<(), DataErrors> {
        if row.len() != self.header.len() { return Err(DataErrors::TooManyValues) }
        //println!("INSERT: {:?}", row);
        Ok(self.rows.push(Row { fields: Vec::from(row) }))
    }

    pub fn get_col(&self, name: &str) -> Result<usize, DataErrors> {
        for i in &self.header {

            if i.0 == name {
                return Ok(i.1);
            }
        }

        return Err(DataErrors::NotFound(format!("No such column '{}'", name)))
    }

    pub fn field_is(mut self, field: &str, match_val: &Field) -> Result<Self, DataErrors> {
        let col = self.get_col(field)?;

        self.rows = self.rows.into_iter().filter(|x| {
            &x.fields[col] == match_val
        }).collect();

        Ok(self)
    }

    pub fn field_is_greater_than(mut self, field: &str, match_val: &Field) -> Result<Self, DataErrors> {
        let col = self.get_col(field)?;

        self.rows = self.rows.into_iter().filter(|x| {
            &x.fields[col] > match_val
        }).collect();

        Ok(self)
    }

    pub fn field_is_less_than(mut self, field: &str, match_val: &Field) -> Result<Self, DataErrors> {
        let col = self.get_col(field)?;

        self.rows = self.rows.into_iter().filter(|x| {
            &x.fields[col] < match_val
        }).collect();

        Ok(self)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    } 

}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.rows {
            for en in &self.header {
                f.write_fmt(format_args!("{}: {}\n", en.0, r.fields[en.1]))?;
            }
            f.write_str("-\n")?;
        }
        Ok(())
    }
}