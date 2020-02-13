use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum BotError {
    DbError(rusqlite::Error),
    ParseError(ParseIntError),
    CustomError(String),
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for BotError {}

impl From<rusqlite::Error> for BotError {
    fn from(err: rusqlite::Error) -> BotError {
        return BotError::DbError(err);
    }
}

impl From<String> for BotError {
    fn from(err: String) -> BotError {
        return BotError::CustomError(err);
    }
}

impl From<ParseIntError> for BotError {
    fn from(err: ParseIntError) -> BotError {
        return BotError::ParseError(err);
    }
}
