use std::error::Error;

pub type GlobalResult<T> = Result<T, Box<dyn Error>>;
