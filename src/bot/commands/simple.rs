use rutebot::requests::ParseMode;

use crate::bot::qbot::RbotParseMode;

use super::QbCommandAction;

pub struct QHelp {}

impl QbCommandAction for QHelp {
    fn name(&self) -> String {
        "/help".to_string()
    }

    fn action_result_to_string(&self) -> String {
        "Help is here".to_string()
    }
}

pub struct UnknownCommand {}

impl QbCommandAction for UnknownCommand {
    fn name(&self) -> String {
        todo!()
    }

    fn action_result_to_string(&self) -> String {
        "Unknown command".to_string()
    }
}

pub struct QStart {}

impl QbCommandAction for QStart {
    fn name(&self) -> String {
        "/start".to_string()
    }

    fn action_result_to_string(&self) -> String {
        r#"
Hello! It's Qbittorrent Telegram bot. 
See the commands list using [/help](/help)."#
            .to_string()
    }

    fn parse_mode(&self) -> RbotParseMode {
        Some(ParseMode::Markdown)
    }
}
