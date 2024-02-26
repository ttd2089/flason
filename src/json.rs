use std::error::Error;
use std::io::Read;

#[derive(Debug)]
pub enum JsonToken<'a> {
    True,
    False,
    Number(&'a str),
    String(&'a str),
    BeginArray,
    EndArray,
    BeginObject,
    EndObject,
    Colon,
    Comma,
    Null,
}

pub struct Tokenizer<R: Read> {
    reader: R,
    buf: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl<R: Read> Tokenizer<R> {
    pub fn new(reader: R) -> Tokenizer<R> {
        Tokenizer {
            reader: reader,
            buf: vec![0; 2],
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<Result<JsonToken, Box<dyn Error>>> {
        if self.read_pos == self.write_pos && self.write_pos < self.buf.len() {
            match self.reader.read(&mut self.buf[self.write_pos..]) {
                Ok(0) => return None,
                Err(err) => return Some(Err(Box::new(err))),
                Ok(n) => self.write_pos += n,
            }
        }

        // todo: Expand the buffer or reuse consumed space.
        if self.read_pos == self.write_pos {
            return None;
        }

        if let Some(tok) = match self.buf[self.read_pos] {
            b'[' => Some(JsonToken::BeginArray),
            b']' => Some(JsonToken::EndArray),
            b'{' => Some(JsonToken::BeginObject),
            b'}' => Some(JsonToken::EndObject),
            b':' => Some(JsonToken::Colon),
            b',' => Some(JsonToken::Comma),
            _ => None,
        } {
            self.read_pos += 1;
            return Some(Ok(tok));
        }

        None
    }
}
