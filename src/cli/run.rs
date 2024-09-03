use std::{any::Any, rc::Rc};

use crate::parser::parse::{DateTime, PlayEntry};

type Table = Vec<PlayEntry>;

pub enum Node {
    Display(Box<Node>),
    TitleMatches(Box<Node>, String),
    /// Get songs with more than <u32>ms of playtime
    PlayTimeAbove(Box<Node>, u32),
    /// Get songs before <DateTime>
    Before(Box<Node>, DateTime),
    /// Get songs after <DateTime>
    After(Box<Node>, DateTime),
    /// Get songs between <DateTime> and <DateTime>
    During(Box<Node>, DateTime, DateTime),
    /// Limit the amount of rows in the table to <u16>
    Limit(Box<Node>, u16),
    Table(Table)
}

fn display(t: Box<Table>) -> Table {

    for row in &*t {
        println!("Title: {}\nArtist: {}\nPlayed on: {}\nDEBUG: {:?}\n", row.song, row.artist, row.time, row.debug_info)
    }

    return *t;
}

fn title_matches(t: Box<Table>, s: String) -> Table {
    t.into_iter().filter(|x| x.song.contains(&s.to_lowercase())).collect()
}

fn playtime_above(t: Box<Table>, time: u32) -> Table {
    t.into_iter().filter(|x| x.ms_played > time).collect()
}

fn get_before(t: Box<Table>, date: DateTime) -> Table {
    
    let mut res: Table = Vec::new();
    for entry in t.into_iter() {
        if entry.time > date {
            break;
        }

        res.push(entry);
    }
    res
}

fn get_after(t: Box<Table>, date: DateTime) -> Table {
    
    let mut res: Table = Vec::new();
    for entry in t.into_iter().rev() {
        if entry.time < date {
            break;
        }

        res.push(entry);
    }
    res.reverse();
    res
}

fn limit(mut t: Box<Table>, amount: u16) -> Table {
    t.reverse();
    t.truncate(amount as usize);
    t.reverse();
    return *t;
}

pub fn run(cmd: Node) -> Table {
    match cmd {
        Node::Table(tbl) => tbl,
        Node::Display(tbl) => display(run(*tbl).into()),
        Node::TitleMatches(tbl, filter_string) => title_matches(run(*tbl).into(), filter_string),
        Node::PlayTimeAbove(tbl, time) => playtime_above(run(*tbl).into(), time),
        Node::Before(tbl, timestamp) => get_before(run(*tbl).into(), timestamp),
        Node::After(tbl, timestamp) => get_after(run(*tbl).into(), timestamp),
        Node::Limit(tbl, amount) => limit(run(*tbl).into(), amount),
        Node::During(tbl, before, after) => run(Node::Before(Node::After(tbl, before).into(), after))

    }
}