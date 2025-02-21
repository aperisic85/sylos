use chrono::{DateTime, FixedOffset};
use nom::{IResult, bytes::complete::take_while, character::complete::char, sequence::tuple, combinator::map, multi::many0};

#[derive(Debug)]
pub struct SyslogMessage {
    pub timestamp: Option<String>,
    pub severity_name: String,
    pub hostname: String,
    pub message: String,
}

// New function to parse timestamp with microseconds and timezone
pub fn parse_timestamp(input: &str) -> IResult<&str, Option<String>> {
    // Example: "Feb 21 10:01:32.190714 +00:00"
    let (input, (month, day, hour, minute, second, microsecond, timezone)) = tuple((
        take_while(|c: char| c.is_alphabetic()),  // Month (e.g., "Feb")
        take_while(|c: char| c.is_numeric()),    // Day (e.g., "21")
        take_while(|c: char| c.is_numeric()),    // Hour (e.g., "10")
        take_while(|c: char| c.is_numeric()),    // Minute (e.g., "01")
        take_while(|c: char| c.is_numeric()),    // Second (e.g., "32")
        take_while(|c: char| c.is_numeric()),    // Microseconds (e.g., "190714")
        take_while(|c: char| c != ' '),          // Timezone (e.g., "+00:00")
    ))(input)?;

    let timestamp_str = format!(
        "2025-{}-{}T{}:{}:{}.{:6}+{}",
        month,
        day,
        hour,
        minute,
        second,
        microsecond, // Add microseconds
        timezone     // Add timezone
    );

    // Parse with the full datetime format
    let timestamp = DateTime::parse_from_str(&timestamp_str, "%Y-%b-%dT%H:%M:%S%.6f+%z")
        .map(|dt| dt.format("%+").to_string())  // Using "%+" for full format
        .ok();

    Ok((input, Some(timestamp.unwrap_or_default())))
}

pub fn parse_syslog(input: &str) -> IResult<&str, SyslogMessage> {
    let (input, (_priority, timestamp, severity_name, hostname, message)) = tuple((
        // Priority (e.g., "<134>")
        take_while(|c: char| c != '>'),
        parse_timestamp,
        take_while(|c: char| c != ' '),
        take_while(|c: char| c != ':'),
        take_while(|c: char| c != '\n'),
    ))(input)?;

    Ok((
        input,
        SyslogMessage {
            timestamp,
            severity_name: severity_name.to_string(),
            hostname: hostname.to_string(),
            message: message.to_string(),
        },
    ))
}
