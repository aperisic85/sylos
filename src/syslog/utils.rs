pub fn get_priority_info(priority: u8) -> (u8, String, u8, String) {
    let facility = priority >> 3;
    let severity = priority & 7;

    let facility_names = [
        "kern", "user", "mail", "daemon", "auth", "syslog", "lpr", "news", "uucp", "clock",
        "authpriv", "ftp", "ntp", "audit", "alert", "clock2", "local0", "local1", "local2",
        "local3", "local4", "local5", "local6", "local7",
    ];

    let severity_names = [
        "Emergency",
        "Alert",
        "Critical",
        "Error",
        "Warning",
        "Notice",
        "Informational",
        "Debug",
    ];

    let facility_name = facility_names
        .get(facility as usize)
        .unwrap_or(&"Unknown")
        .to_string();
    let severity_name = severity_names
        .get(severity as usize)
        .unwrap_or(&"Unknown")
        .to_string();

    (facility, facility_name, severity, severity_name)
}
