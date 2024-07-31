use crate::error::{
    Error,
    Result,
};

#[derive(Debug)]
pub struct Args {

}

impl Args {
    pub fn parse_from() -> Result<Args> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Config {
    pub worker_count: usize,
}

impl Config {
    pub fn load(args: &Args) -> Result<Config> {
        todo!()
    }
}