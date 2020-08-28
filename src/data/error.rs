use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum BotError {
    DbError(sqlx::Error),
    ParseError(ParseIntError),
    SerenityError(serenity::Error),
    CustomError(String),
    NoneError,
    PrefixNotFound,
    LogTypeNotFound,
    LogTypeDisabled,
    NoRecordYet,
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut msg = String::new();
        msg.push_str("[BotError] ");
        let error = match self {
            BotError::DbError(e) => e.to_string(),
            BotError::ParseError(e) => e.to_string(),
            BotError::SerenityError(e) => e.to_string(),
            BotError::CustomError(e) => e.to_string(),
            BotError::NoneError => "Unwrapped a None Option".into(),
            BotError::PrefixNotFound => "Prefix was not found".into(),
            BotError::LogTypeNotFound => "Log type was not found".into(),
            BotError::LogTypeDisabled => "This log type is disabled".into(),
            BotError::NoRecordYet => "User has no score record yet".into(),
        };
        msg.push_str(&error);
        f.write_str(&msg)
    }
}

impl Error for BotError {}

impl From<sqlx::Error> for BotError {
    fn from(err: sqlx::Error) -> BotError {
        BotError::DbError(err)
    }
}

impl From<String> for BotError {
    fn from(err: String) -> BotError {
        BotError::CustomError(err)
    }
}

impl From<ParseIntError> for BotError {
    fn from(err: ParseIntError) -> BotError {
        BotError::ParseError(err)
    }
}

impl From<serenity::Error> for BotError {
    fn from(err: serenity::Error) -> BotError {
        BotError::SerenityError(err)
    }
}
