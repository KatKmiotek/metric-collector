use chrono::Duration;

pub trait DurationFormatter {
    fn format_duration(&self) -> String;
}

impl DurationFormatter for Duration {
    fn format_duration(&self) -> String {
        let total_seconds = self.num_seconds();
        let hours = total_seconds / 3600;
        let remaining_minutes = (total_seconds % 3600) / 60;
        let remaining_seconds = total_seconds % 60;
        format!("{}.{}.{}", hours, remaining_minutes, remaining_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_zero_duration() {
        let duration = Duration::zero();
        assert_eq!(duration.format_duration(), "0.0.0");
    }

    #[test]
    fn test_format_seconds_only() {
        let duration = Duration::seconds(45);
        assert_eq!(duration.format_duration(), "0.0.45");
    }

    #[test]
    fn test_format_minutes_only() {
        let duration = Duration::minutes(30);
        assert_eq!(duration.format_duration(), "0.30.0");
    }

    #[test]
    fn test_format_hours_only() {
        let duration = Duration::hours(2);
        assert_eq!(duration.format_duration(), "2.0.0");
    }

    #[test]
    fn test_format_large_duration() {
        let duration = Duration::hours(24) + Duration::minutes(59) + Duration::seconds(59);
        assert_eq!(duration.format_duration(), "24.59.59");
    }

    #[test]
    fn test_format_complex_rollover() {
        let duration = Duration::seconds(3665);
        assert_eq!(duration.format_duration(), "1.1.5");
    }
}
