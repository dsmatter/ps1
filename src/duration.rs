use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    if duration.as_nanos() < 1000 {
        format!("{}ns", duration.as_nanos())
    } else if duration.as_micros() < 1000 {
        format!("{}µs", duration.as_micros())
    } else if duration.as_millis() < 1000 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_secs() < 2 * 60 {
        format!("{:.2}s", duration.as_secs_f32())
    } else if duration.as_secs() < 2 * 60 * 60 {
        let mins = duration.as_secs() / 60;
        let secs = duration.as_secs() % 60;
        format!("{}:{:02}min", mins, secs)
    } else {
        let hours = duration.as_secs() / 3600;
        let mins = (duration.as_secs() / 60) % 60;
        let secs = duration.as_secs() % 60;
        format!("{}:{:02}:{:02}h", hours, mins, secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_sample_tests() {
        assert_eq!(format_duration(Duration::from_nanos(42)), "42ns");
        assert_eq!(format_duration(Duration::from_micros(420)), "420µs");
        assert_eq!(format_duration(Duration::from_millis(230)), "230ms");
        assert_eq!(format_duration(Duration::from_millis(42230)), "42.23s");
        assert_eq!(format_duration(Duration::from_millis(422300)), "7:02min");
        assert_eq!(format_duration(Duration::from_secs(4223)), "70:23min");
        assert_eq!(format_duration(Duration::from_secs(42230)), "11:43:50h");
        assert_eq!(format_duration(Duration::from_secs(42 * 3600)), "42:00:00h");
    }
}
