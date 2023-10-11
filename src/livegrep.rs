use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct LiveGrepLine {
    line: String,

    date: NaiveDateTime,
}

const TIMEZONE_LEN: usize = 6;

impl LiveGrepLine {
    fn parse_from_str(str: &str) -> anyhow::Result<LiveGrepLine> {
        let start_date_index = str.to_string().find("[");
        if start_date_index.is_none() {
            return Err(anyhow!("Invalid date formatting"));
        }

        let start_date_index = start_date_index.unwrap();
        let first_substring = &str.to_string()[0..start_date_index];
        let rest = &str.to_string()[start_date_index + 1..str.len()];

        let end_date_index = rest.find("]");
        if end_date_index.is_none() {
            return Err(anyhow!("Invalid date formatting"));
        }

        // All date times are from -4 timezone, so we can just omit that part
        let end_date_index = end_date_index.unwrap();
        let date_substring = &rest.to_string()[0..end_date_index - TIMEZONE_LEN];
        let second_half = &rest.to_string()[end_date_index + 1..rest.len()];

        let matched_string = first_substring.to_string() + second_half;

        let only_date = &date_substring[0..date_substring.find(":").unwrap()];
        let date_format = "%d/%b/%Y";
        let date = NaiveDate::parse_from_str(only_date, date_format)?;

        let time_format = "%H:%M:%S";
        let only_time =
            &date_substring[date_substring.find(":").unwrap() + 1..date_substring.len()];
        let parsed_time = NaiveTime::parse_from_str(only_time, time_format)?;
        let parsed_datetime = NaiveDateTime::new(date, parsed_time);

        Ok(LiveGrepLine {
            line: matched_string,
            date: parsed_datetime,
        })
    }

    fn line_matches(&self, regex: String) -> anyhow::Result<bool> {
        let re = Regex::new(regex.as_str())?;
        Ok(re.is_match(self.line.as_str()))
    }

    fn contains_date(&self, date_range: DateRange) -> bool {
        self.date.date() >= date_range.start_date && self.date.date() <= date_range.end_date
    }
}

#[derive(Clone, Debug)]
pub struct LiveGrep {
    pub regex: String,
    pub date_range: DateRange,
}

impl LiveGrep {
    fn scan_file(&self, file: PathBuf) -> anyhow::Result<Vec<String>> {
        let opened_file = File::open(file)?;
        let reader = BufReader::new(opened_file);
        let mut result = Vec::new();

        for line in reader.lines() {
            let l = line?;

            let grep_line = LiveGrepLine::parse_from_str(l.as_str())?;

            if grep_line.line_matches(self.regex.clone())?
                && grep_line.contains_date(self.date_range.clone())
            {
                result.push(l)
            }
        }

        Ok(result)
    }

    pub fn scan_directory(&self, data_dir: PathBuf) -> anyhow::Result<Vec<String>> {
        let directory = fs::read_dir(data_dir)?;
        let mut result = Vec::new();
        for entry in directory {
            let dir_entry = entry?;
            let file_type = dir_entry.file_type()?;
            if file_type.is_dir() {
                let mut lines = self.scan_directory(dir_entry.path())?;
                result.append(&mut lines);
            } else {
                let mut lines = self.scan_file(dir_entry.path())?;
                result.append(&mut lines);
            }
        }

        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct DateRange {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl DateRange {
    pub fn parse_date_range(date: String) -> anyhow::Result<Self> {
        let dates: Vec<&str> = date.split("-").collect();
        if dates.len() != 2 {
            return Err(anyhow!("Invalid date range specified."));
        }

        let date_format = "%d/%b/%Y";
        let start_date = NaiveDate::parse_from_str(dates[0], date_format)?;
        let end_date = NaiveDate::parse_from_str(dates[1], date_format)?;
        Ok(DateRange {
            start_date,
            end_date,
        })
    }
}
