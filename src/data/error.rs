use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum BotError {
    DbError(sqlx::Error),
    ParseError(ParseIntError),
    CustomError(String),
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:#?}", self)
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
