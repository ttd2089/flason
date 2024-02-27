use std::io;
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

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    UnexpectedEof,
    JsonError(String),
}

pub struct Tokenizer<R: Read> {
    // todo: Track position to include with tokens / errors.
    reader: R,
    buf: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl<R: Read> Tokenizer<R> {
    pub fn new(reader: R) -> Tokenizer<R> {
        Tokenizer {
            reader: reader,
            buf: vec![0; 1],
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<Result<JsonToken, Error>> {
        match self.scan_to_content() {
            Ok(0) => return None,
            Err(err) => return Some(Err(err)),
            _ => {}
        }

        let res = match self.buf[self.read_pos] {
            b'[' => self.parse_literal("[", JsonToken::BeginArray),
            b']' => self.parse_literal("]", JsonToken::EndArray),
            b'{' => self.parse_literal("{", JsonToken::BeginObject),
            b'}' => self.parse_literal("}", JsonToken::EndObject),
            b':' => self.parse_literal(":", JsonToken::Colon),
            b',' => self.parse_literal(",", JsonToken::Comma),
            b't' => self.parse_literal("true", JsonToken::True),
            b'f' => self.parse_literal("false", JsonToken::False),
            b'n' => self.parse_literal("null", JsonToken::Null),
            b'"' => self.parse_string(),
            b'0'..=b'9' | b'-' => self.parse_number(),
            _ => Err(Error::JsonError(String::from("invalid byte"))),
        };

        Some(res)
    }

    fn parse_literal<'a>(
        &'a mut self,
        literal: &str,
        token: JsonToken<'a>,
    ) -> Result<JsonToken, Error> {
        // The first byte always matches because we used it to determine which literal to parse so
        // we can start parsing at the second byte otherwise.

        let mut matched = 1;
        self.read_pos += 1;

        while matched < literal.len() {
            if self.buffered_unread().len() == 0 {
                match self.read_to_front_of_buffer() {
                    Ok(0) => return Err(Error::UnexpectedEof),
                    Err(err) => return Err(Error::IoError(err)),
                    Ok(n) => self.write_pos += n,
                }
            }
            if self.buffered_unread()[0] == literal.as_bytes()[matched] {
                self.read_pos += 1;
                matched += 1;
                continue;
            }

            return Err(Error::JsonError(String::from("invalid literal")));
        }
        Ok(token)
    }

    fn parse_string(&mut self) -> Result<JsonToken, Error> {
        Err(Error::JsonError(String::from(
            "parser not implemented for string",
        )))
    }

    fn parse_number(&mut self) -> Result<JsonToken, Error> {
        Err(Error::JsonError(String::from(
            "parser not implemented for number",
        )))
    }

    fn scan_to_content(&mut self) -> Result<usize, Error> {
        self.discard_whitespace();

        while self.buffered_unread().len() == 0 {
            // All of the data in the buffer has been consumed by returned tokens. This is most
            // likely to occur on the first call to next or after the whole stream has been parsed
            // successfully, but it could also mean that a read into the buffer just happened to
            // align with a token or whitespace boundary. Either way we should attempt a read to
            // the front of the buffer to get as much data as we can without allocating.
            match self.read_to_front_of_buffer() {
                // Nothing left to read means we parsed everything successfully.
                Ok(0) => return Ok(0),
                Err(err) => return Err(Error::IoError(err)),
                Ok(n) => self.write_pos += n,
            }

            self.discard_whitespace();
        }

        Ok(self.write_pos - self.read_pos)
    }

    fn discard_whitespace(&mut self) {
        while self.buffered_unread().len() > 0 {
            match self.buffered_unread()[0] {
                // Per https://datatracker.ietf.org/doc/html/rfc8259 a carriage return can
                // appear anywhere as insignificant whitespace, not only before a line feed.
                b' ' | b'\t' | b'\n' | b'\r' => {
                    self.read_pos += 1;
                    continue;
                }
                _ => break,
            }
        }
    }

    fn read_to_front_of_buffer(&mut self) -> std::io::Result<usize> {
        self.read_pos = 0;
        self.write_pos = 0;
        loop {
            match self.reader.read(&mut self.buf[..]) {
                ok @ Ok(_) => return ok,
                Err(err) => match err.kind() {
                    io::ErrorKind::Interrupted => continue,
                    _ => return Err(err),
                },
            }
        }
    }

    fn buffered_unread<'a>(&'a self) -> &[u8] {
        &self.buf[self.read_pos..self.write_pos]
    }
}
