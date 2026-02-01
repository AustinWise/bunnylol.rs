/// Google Cloud Console command handler
/// Supports: gcp -> redirects to Google Cloud Console
use crate::commands::bunnylol_command::{BunnylolCommand, BunnylolCommandInfo};

pub struct GoogleCloudCommand;

impl BunnylolCommand for GoogleCloudCommand {
    const BINDINGS: &'static [&'static str] = &["gcp", "gcloud", "pantheon"];

    fn process_args(_args: &str) -> String {
        "https://console.cloud.google.com/".to_string()
    }

    fn get_info() -> BunnylolCommandInfo {
        BunnylolCommandInfo::new(
            Self::BINDINGS,
            "Navigate to Google Cloud Console",
            "gcp",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_cloud_command() {
        assert_eq!(
            GoogleCloudCommand::process_args("gcp"),
            "https://console.cloud.google.com/"
        );
    }

    #[test]
    fn test_google_cloud_command_with_args() {
        assert_eq!(
            GoogleCloudCommand::process_args("gcp some args"),
            "https://console.cloud.google.com/"
        );
    }
}
