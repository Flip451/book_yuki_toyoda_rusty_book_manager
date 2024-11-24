pub trait RedisKey {
    type Value: RedisValue + TryFrom<String, Error = RedisValueError>;
    fn inner(&self) -> String;
}

pub trait RedisValue {
    fn inner(&self) -> String;
}

#[derive(Debug, thiserror::Error)]
pub enum RedisValueError {
    #[error("parsing error: {0}")]
    ParsingError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type RedisValueResult<T> = Result<T, RedisValueError>;
