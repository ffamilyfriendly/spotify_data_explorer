use super::parse::DateTime;

pub fn quick_date(year: u16, month: u8, day: u8) -> DateTime {
    DateTime { day: day, month: month, year: year, minute: 0, hour: 0 }
}