use std::fmt;

use crate::resp::{RespError, RespType};

struct RespCommandFrame {
    builder: Option<CommandBuilder>,
}

struct CommandBuilder {
    parts: Vec<RespType>,
    num_parts: usize,
    parsed_parts: usize,
}

impl CommandBuilder {
    pub fn new(num_parts: usize) -> CommandBuilder {
        CommandBuilder {
            parts: vec![],
            num_parts,
            parsed_parts: 0,
        }
    }

    pub fn add_part(&mut self, part: RespType) {
        self.parts.push(part);
        self.parsed_parts += 1;
    }

    pub fn all_parts_received(&self) -> bool {
        self.num_parts == self.parsed_parts
    }

    pub fn build(&self) -> Vec<RespType> {
        self.parts.clone()
    }
}

#[derive(Debug)]
pub struct FrameError {
    err: RespError,
}

impl FrameError {
    pub fn from(err: RespError) -> FrameError {
        FrameError { err }
    }
}

impl std::error::Error for FrameError {}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}
