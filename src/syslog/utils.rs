pub fn get_priority_info(priority: u8) -> (u8, String, u8, String) {
    let facility = priority >> 3;
    let severity = priority & 7;

    // Use match for facilities to ensure no out-of-bound access
    let facility_name = match facility {
        0 => "kern",
        1 => "user",
        2 => "mail",
        3 => "daemon",
        4 => "auth",
        5 => "syslog",
        6 => "lpr",
        7 => "news",
        8 => "uucp",
        9 => "clock",
        10 => "authpriv",
        11 => "ftp",
        12 => "ntp",
        13 => "audit",
        14 => "alert",
        15 => "clock2",
        16 => "local0",
        17 => "local1",
        18 => "local2",
        19 => "local3",
        20 => "local4",
        21 => "local5",
        22 => "local6",
        23 => "local7",
        _ => "Unknown", // Default to Unknown for invalid facilities
    }.to_string();

    // Use match for severities to ensure no out-of-bound access
    let severity_name = match severity {
        0 => "Emergency",
        1 => "Alert",
        2 => "Critical",
        3 => "Error",
        4 => "Warning",
        5 => "Notice",
        6 => "Informational",
        7 => "Debug",
        _ => "Unknown", // Default to Unknown for invalid severities
    }.to_string();

    (facility, facility_name, severity, severity_name)
}
