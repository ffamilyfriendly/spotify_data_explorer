use crate::parser::parse::{DateTime, PlayEntry};

type Table = Vec<PlayEntry>;

pub enum Node<'a> {
    Display(&'a Node<'a>),
    TitleMatches(&'a Node<'a>, String),
    /// Get songs with more than <u32>ms of playtime
    PlayTimeAbove(&'a Node<'a>, u32),
    /// Get songs before <DateTime>
    Before(&'a Node<'a>, &'a DateTime),
    /// Get songs after <DateTime>
    After(&'a Node<'a>, &'a DateTime),
    /// Get songs between <DateTime> and <DateTime>
    During(&'a Node<'a>, DateTime, DateTime),
    Table(Table),
}

fn display(t: Table) -> Table {
    for row in &t {
        println!(
            "Title: {}\nArtist: {}\nPlayed on: {}\nDEBUG: {:?}\n",
            row.song, row.artist, row.time, row.debug_info
        )
    }

    t
}

fn title_matches(t: Table, s: &str) -> Table {
    let sl = s.to_lowercase();

    t.into_iter().filter(|x| x.song.contains(&sl)).collect()
}

fn playtime_above(t: Table, time: u32) -> Table {
    t.into_iter().filter(|x| x.ms_played > time).collect()
}

fn get_before(t: Table, date: &DateTime) -> Table {
    t.into_iter()
        .take_while(|entry| &entry.time > date)
        .collect()
}

fn get_after(t: Table, date: &DateTime) -> Table {
    let mut res: Table = t
        .into_iter()
        .rev()
        .take_while(|entry| &entry.time < date)
        .collect();

    res.reverse();
    res
}

pub fn run(cmd: &Node) -> Table {
    match cmd {
        Node::Table(tbl) => tbl.clone(),
        Node::Display(tbl) => display(run(tbl)),
        Node::TitleMatches(tbl, filter_string) => title_matches(run(tbl), filter_string),
        Node::PlayTimeAbove(tbl, time) => playtime_above(run(tbl), *time),
        Node::Before(tbl, timestamp) => get_before(run(tbl), timestamp),
        Node::After(tbl, timestamp) => get_after(run(tbl), timestamp),
        Node::During(tbl, before, after) => {
            let parent_node = Node::After(tbl, before);
            let node = Node::Before(&parent_node, after);

            run(&node)
        }
    }
}
