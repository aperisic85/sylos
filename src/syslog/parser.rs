use crate::syslog::utils;  // Import utils module

use chrono::{NaiveDateTime, Datelike, Utc};
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{digit1, space1},
    combinator::{map_res, opt},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
pub struct SyslogMessage {
    pub timestamp: Option<NaiveDateTime>,
    pub hostname: String,
    pub facility: u8,
    pub facility_name: String,
    pub severity: u8,
    pub severity_name: String,
    pub message: String,
}

// Parse syslog priority (e.g., "<134>")
fn parse_priority(input: &str) -> IResult<&str, (u8, String, u8, String)> {
    let (input, priority) = delimited(tag("<"), map_res(digit1, str::parse::<u8>), tag(">"))(input)?;
    Ok((input, utils::get_priority_info(priority)))
}

// Parse timestamp (e.g., "Feb 21 10:14:32")
fn parse_timestamp(input: &str) -> IResult<&str, Option<NaiveDateTime>> {
    let (input, timestamp) = opt(tuple((
        take_while(|c: char| c.is_alphabetic()),  // Month (e.g., "Feb")
        space1,
        take_while(|c: char| c.is_digit(10)),      // Day (e.g., "21")
        space1,
        take_until(" "),                           // Time (e.g., "10:14:32")
        space1,
    )))(input)?;

    if let Some((month, _, day, _, time, _)) = timestamp {
        let formatted_ts = format!("{} {} {} {}", Utc::now().year(), month, day, time);
        if let Ok(parsed_time) = NaiveDateTime::parse_from_str(&formatted_ts, "%Y %b %d %H:%M:%S") {
            return Ok((input, Some(parsed_time)));
        }
    }

    Ok((input, None))
}

// Parse hostname (e.g., "myhost")
fn parse_hostname(input: &str) -> IResult<&str, String> {
    let (input, hostname) = take_until(" ")(input)?;
    let (input, _) = space1(input)?;
    Ok((input, hostname.to_string()))
}

// Parse syslog message content
fn parse_message(input: &str) -> IResult<&str, String> {
    Ok((input, input.trim().to_string()))
}

// Complete syslog parser
pub fn parse_syslog(input: &str) -> IResult<&str, SyslogMessage> {
    let (input, (facility, facility_name, severity, severity_name)) = parse_priority(input)?;
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, hostname) = parse_hostname(input)?;
    let (input, message) = parse_message(input)?;

    Ok((
        input,
        SyslogMessage {
            timestamp,
            hostname,
            facility,
            facility_name,
            severity,
            severity_name,
            message,
        },
    ))
}
