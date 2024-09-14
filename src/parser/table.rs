use std::{collections::HashMap, fmt::{Debug, Display, Write}, io::Read, ops::Range};

use super::parse::DateTime;

#[derive(PartialEq, PartialOrd, Clone, Debug, Hash, Eq)]
pub enum Field {
    Date(DateTime),
    String(String),
    Number(u64),
    Bool(bool)
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Field::String(s) => s,
            Field::Number(n) => &n.to_string(),
            Field::Date(d) => &format!("{}", d),
            Field::Bool(b) => &format!("{}", b)
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

impl From<u64> for Field {
    fn from(value: u64) -> Self {
        Field::Number(value)
    }
}

impl From<DateTime> for Field {
    fn from(value: DateTime) -> Self {
        Field::Date(value)
    }
}

impl From<bool> for Field {
    fn from(value: bool) -> Self {
        Field::Bool(value)
    }
}

#[derive(Clone)]
pub struct Row {
    pub fields: Vec<Field>
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_arr: Vec<String> = self.fields.iter().map(|item| item.to_string()).collect();
        f.write_str(&str_arr.join("|"))
    }
}

pub static BIG_HISTORY_TABLE: [&str; 21] = ["time", "username", "platform", "msplayed", "country", "ip_addr", "user_agent", "song", "artist", "album", "track_uri", "episode_name", "episode_show_name", "episode_uri", "reason_start", "reason_end", "shuffle", "skipped", "offline", "offline_timestamp", "incognito_mode"];

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

    pub fn field_in_range(mut self, field: &str, lower: &Field, upper: &Field) -> Result<Self, DataErrors> {
        let col = self.get_col(field)?;

        self.rows = self.rows.into_iter().filter(|x| {
            &x.fields[col] > lower && &x.fields[col] <= upper
        }).collect();

        Ok(self)
    }

    pub fn group_by(&self, field: &str) -> Result<Table, DataErrors> {
        let col = self.get_col(field)?;

        let mut cache = HashMap::new();

        for row in &self.rows {
            let field = &row.fields[col];

            cache.entry(field.clone()).and_modify(|count| *count += 1).or_insert(1);
        };
        
        let rows = cache.into_iter().map(|(x, y)| Row { fields: vec![ x.clone(), Field::Number(y as u64) ] }).collect();

        let table = Table {
            header: vec![(field.to_owned(), 0), ("COUNT".to_owned(), 1)],
            rows: rows
        };
        
        Ok(table)
    }

    pub fn select<const T: usize>(self, cols: [&str; T]) -> Self {
        Table {
            header: self.header.into_iter().filter(|x| cols.contains(&x.0.as_str())).collect(),
            rows: self.rows
        }
    }

    pub fn sort_by(mut self, field: &str) -> Result<Self, DataErrors> {
        let col = self.get_col(field)?;
        self.rows.sort_by(| a, b | a.fields[col].partial_cmp(&b.fields[col]).expect("CANT ORDER"));

        let ordered = Table {
            header: self.header,
            rows: self.rows
        };

        Ok(ordered)
    }

    pub fn row_at(&self, index: usize) -> Option<Row> {
        let first = match self.rows.get(index) {
            Some(s) => s,
            None => return None
        };

        let fields: Vec<Field> = self.header.iter().map(|(_name, col)| first.fields[*col].clone()).collect();

        Some(Row {
            fields
        })
    }

    pub fn take(&self, range: Range<i32>) -> Vec<Row> {
        let mut res = Vec::new();
        for i in range {
            match self.row_at(i as usize) {
                Some(i) => res.push(i),
                None => {}
            }
        }

        res
    }

    pub fn take_first(&self) -> Option<Row> {
        self.row_at(0)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    } 

}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.rows {
            let fields: Vec<&Field> = self.header.iter().map(|(_name, col)| &r.fields[*col]).collect();
            let as_str: Vec<String> = fields.into_iter().map(|f| f.to_string()).collect();
            f.write_str(&format!("{}\n", as_str.join("|")))?;
        }
        Ok(())
    }
}