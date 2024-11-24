use chrono::Duration;

pub trait DurationFormatter {
    fn format_duration(&self) -> String;
}

impl DurationFormatter for Duration {
    fn format_duration(&self) -> String {
        let hours = self.num_hours();
        let minutes = self.num_minutes();
        let seconds = self.num_seconds() % 60;
        format!("{}.{}.{}", hours, minutes, seconds)
    }
}
