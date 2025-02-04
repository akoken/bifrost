use crate::resp::RespType;
use crate::error::BifrostError;
use crate::commands::{
    Command, PingCommand, EchoCommand, GetCommand, 
    SetCommand, DelCommand, ExistsCommand, IncrCommand, DecrCommand
};

pub fn parse_command(request: RespType) -> Result<Box<dyn Command>, BifrostError> {
    match request {
        RespType::Array(array) => {
            if let Some(RespType::BulkString(command)) = array.first() {
                match command.to_uppercase().as_str() {
                    "PING" => Ok(Box::new(PingCommand)),
                    "ECHO" => {
                        let message = array.get(1)
                            .ok_or_else(|| BifrostError::CommandError(
                                "ERR wrong number of arguments for 'echo' command".to_string()
                            ))?;
                        Ok(Box::new(EchoCommand(message.clone())))
                    }
                    "GET" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            Ok(Box::new(GetCommand(key.clone())))
                        } else {
                            Err(BifrostError::CommandError(
                                "ERR wrong number of arguments for 'get' command".to_string()
                            ))
                        }
                    }
                    "SET" => {
                        match (array.get(1), array.get(2)) {
                            (Some(RespType::BulkString(key)), Some(value)) => {
                                Ok(Box::new(SetCommand {
                                    key: key.clone(),
                                    value: value.clone(),
                                }))
                            }
                            _ => Err(BifrostError::CommandError(
                                "ERR wrong number of arguments for 'set' command".to_string()
                            ))
                        }
                    }
                    "DEL" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            Ok(Box::new(DelCommand(key.clone())))
                        } else {
                            Err(BifrostError::CommandError("ERR wrong number of arguments for 'del' command".to_string()))
                        }
                    }
                    "EXISTS" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            Ok(Box::new(ExistsCommand(key.clone())))
                        } else {
                            Err(BifrostError::CommandError("ERR wrong number of arguments for 'exists' command".to_string()))
                        }
                    }
                    "INCR" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            Ok(Box::new(IncrCommand(key.clone())))
                        } else {
                            Err(BifrostError::CommandError("ERR wrong number of arguments for 'incr' command".to_string()))
                        }
                    }
                    "DECR" => {
                        if let Some(RespType::BulkString(key)) = array.get(1) {
                            Ok(Box::new(DecrCommand(key.clone())))
                        } else {
                            Err(BifrostError::CommandError("ERR wrong number of arguments for 'decr' command".to_string()))
                        }
                    }
                    _ => Err(BifrostError::CommandError("ERR unknown command".to_string()))
                }
            } else {
                Err(BifrostError::ProtocolError("ERR invalid request".to_string()))
            }
        }
        _ => Err(BifrostError::ProtocolError("ERR invalid request".to_string()))
    }
} 