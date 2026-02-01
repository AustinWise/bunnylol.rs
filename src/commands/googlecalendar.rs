/// Google Calendar command handler
/// Supports: gcal -> redirects to Google Calendar
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct GoogleCalendarCommand;

impl BunnylolCommand for GoogleCalendarCommand {
    const BINDINGS: &'static [&'static str] = &["gcal", "c"];

    fn process_args(_args: &str) -> String {
        "https://calendar.google.com/".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Google Calendar",
            "gcal",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_calendar_command() {
        assert_eq!(
            GoogleCalendarCommand::process_args("gcal"),
            "https://calendar.google.com/"
        );
    }

    #[test]
    fn test_google_calendar_command_with_args() {
        assert_eq!(
            GoogleCalendarCommand::process_args("gcal some args"),
            "https://calendar.google.com/"
        );
    }
}
