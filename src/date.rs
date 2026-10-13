pub fn format_date_numeric(date: &str) -> String {
    if date.is_empty() {
        return String::new();
    }
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() < 2 {
        return date.to_string();
    }
    let year = parts[0];
    let month = parts[1];
    if let Ok(m) = month.parse::<u32>() {
        format!("{:02}/{}", m, year)
    } else {
        format!("{}/{}", month, year)
    }
}

pub fn format_date_localized(date: &str, lang: &str) -> String {
    if date.is_empty() {
        return String::new();
    }
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() < 2 {
        return date.to_string();
    }
    let year = parts[0];
    let month = parts[1];

    let m_num = month.parse::<usize>().unwrap_or(0);
    if m_num >= 1 && m_num <= 12 {
        let is_fr = lang.to_lowercase() == "fr";
        let months_en = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let months_fr = [
            "janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.", "août", "sept.", "oct.",
            "nov.", "déc.",
        ];
        let m_str = if is_fr {
            months_fr[m_num - 1]
        } else {
            months_en[m_num - 1]
        };
        format!("{} {}", m_str, year)
    } else {
        format_date_numeric(date)
    }
}

pub fn format_relative_time(timestamp: u64, now: u64) -> String {
    if timestamp == 0 || now < timestamp {
        return "Recently".to_string();
    }
    let diff = now - timestamp;
    if diff < 60 {
        "Just now".to_string()
    } else if diff < 3600 {
        let mins = diff / 60;
        format!("{}m ago", mins)
    } else if diff < 86400 {
        let hours = diff / 3600;
        format!("{}h ago", hours)
    } else if diff < 86400 * 30 {
        let days = diff / 86400;
        format!("{}d ago", days)
    } else {
        let months = diff / (86400 * 30);
        format!("{}mo ago", months)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_relative_time() {
        assert_eq!(format_relative_time(1000, 1020), "Just now");
        assert_eq!(format_relative_time(1000, 1180), "3m ago");
        assert_eq!(format_relative_time(1000, 8200), "2h ago");
        assert_eq!(format_relative_time(1000, 1000 + 86400 * 3), "3d ago");
    }

    #[test]
    fn test_format_date_numeric() {
        assert_eq!(format_date_numeric("2024-05"), "05/2024");
        assert_eq!(format_date_numeric("2024-5"), "05/2024");
        assert_eq!(format_date_numeric(""), "");
        assert_eq!(format_date_numeric("2024"), "2024");
    }

    #[test]
    fn test_format_date_localized() {
        assert_eq!(format_date_localized("2024-05", "en"), "May 2024");
        assert_eq!(format_date_localized("2024-05", "fr"), "mai 2024");
        assert_eq!(format_date_localized("2024-01", "fr"), "janv. 2024");
        assert_eq!(format_date_localized("2024-12", "en"), "Dec 2024");
        assert_eq!(format_date_localized("", "en"), "");
    }
}

