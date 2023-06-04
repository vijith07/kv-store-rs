use anyhow::{Error, Result};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

const CR: u8 = b'\r';
const LF: u8 = b'\n';

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Resp {
    Null,
    SimpleString(String),
    Error(String),
    BulkString(String),
    Integer(i64),
    Array(Vec<Resp>),
}

impl Resp {
    pub fn to_command(&self) -> Result<(String, Vec<Resp>)> {
        match self {
            Resp::Array(items) => {
                let mut iter = items.iter();
                let command = match iter.next() {
                    Some(Resp::BulkString(s)) => s.clone(),
                    _ => {
                        return Err(Error::msg("Not a bulk string"));
                    }
                };
                let args = iter.map(|item| item.clone()).collect();
                Ok((command, args))
            }
            _ => {
                return Err(Error::msg("Not an array"));
            }
        }
    }


    // encode the resp
    pub fn encode(&self) -> String {
        match self {
            Resp::Null => "$-1\r\n".to_string(),
            Resp::Integer(i) => format!(":{}\r\n", i),
            Resp::SimpleString(s) => format!("+{}\r\n", s),
            Resp::Error(s) => format!("-{}\r\n", s),
            Resp::BulkString (s) => format!("${}\r\n{}\r\n", s.len(), s),
            _ => panic!("Not implemented for {:?}", self),
        }
    }
}

pub struct RespCodec {
    stream: TcpStream,
    buffer: BytesMut,
}

impl RespCodec {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(1024),
        }
    }

    pub async fn read_resp(&mut self) -> Result<Option<Resp>> {
        loop {
            let bytes_read = self.stream.read_buf(&mut self.buffer).await?;
            if bytes_read == 0 {
                return Ok(None);
            }
            if let Some((resp, _bytes_consumed)) = parse_message(self.buffer.clone())? {
                return Ok(Some(resp));
            }
        }
    }

    pub async fn write_resp(&mut self, resp: Resp) -> Result<()> {
        self.stream.write_all(resp.encode().as_bytes()).await?;
        Ok(())
    }
}

fn parse_message(buffer: BytesMut) -> Result<Option<(Resp, usize)>> {
    match buffer[0] as char {

        '+' => decode_simple_string(buffer),

        '*' => decode_array(buffer),

        '$' => decode_bulk_string(buffer),

        _ => Err(Error::msg("unrecognised message type")),

    }
}

//decode simple string
fn decode_simple_string(buffer: BytesMut) -> Result<Option<(Resp, usize)>> {
    if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
        let str = parse_string(line)?;
        return Ok(Some((Resp::SimpleString(str), len + 1)));
    }
    return Ok(None);
}

//decode array
fn decode_array(buffer: BytesMut) -> Result<Option<(Resp, usize)>> {
    let (array_length, mut bytes_consumed) =
        if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
            let array_length = parse_integer(line)?;
            (array_length, len + 1)
        } else {
            return Ok(None);
        };
    let mut items: Vec<Resp> = Vec::new();
    for _ in 0..array_length {
        if let Some((v, len)) = parse_message(BytesMut::from(&buffer[bytes_consumed..]))? {
            items.push(v);
            bytes_consumed += len;
        } else {
            return Ok(None);
        }
    }
    return Ok(Some((Resp::Array(items), bytes_consumed)));
}

//decode bulk string
fn decode_bulk_string(buffer: BytesMut) -> Result<Option<(Resp, usize)>> {
    let (bulk_length, bytes_consumed) = if let Some((line, len)) = read_until_crlf(&buffer[1..]) {

        let bulk_length = parse_integer(line)?;
        (bulk_length, len + 1)
    } else {
        return Ok(None);
    };
    let end_of_bulk = bytes_consumed + (bulk_length as usize);
    let end_of_bulk_line = end_of_bulk + 2;
    return if end_of_bulk_line <= buffer.len() {
        Ok(Some((
            Resp::BulkString(parse_string(&buffer[bytes_consumed..end_of_bulk])?),
            end_of_bulk_line,
        )))

    } else {
        Ok(None)
    };

}
// read until crlf
fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
    let mut index = 0;
    while index + 1 < buffer.len() {
        if buffer[index] == CR && buffer[index + 1] == LF {
            return Some((&buffer[..index], index + 2));
        }
        index += 1;
    }
    None
}

fn parse_string(bytes: &[u8]) -> Result<String> {
    String::from_utf8(bytes.to_vec()).map_err(|_| Error::msg("Could not parse string"))

}

fn parse_integer(bytes: &[u8]) -> Result<i64> {
    let str_integer = parse_string(bytes)?;
    (str_integer.parse::<i64>()).map_err(|_| Error::msg("Could not parse integer"))
}