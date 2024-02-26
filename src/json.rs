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
            Err(err) => return Some(Err(err)),
            Ok(false) => return None,
            // todo: Figure out why this is necessary.
            Ok(true) => (),
        }

        // todo: Factor this out to a function.
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

        Some(match self.buf[self.read_pos] {
            b't' => self.parse_literal("true", JsonToken::True),
            b'f' => self.parse_literal("false", JsonToken::False),
            b'n' => self.parse_literal("null", JsonToken::Null),
            // todo: Factor out unexpected char error handler.
            _ => Err(Error::JsonError("unexpected character".to_owned())),
        })
    }

    fn parse_literal<'a>(
        &'a mut self,
        literal: &str,
        token: JsonToken<'a>,
    ) -> Result<JsonToken, Error> {
        let mut matched = 0;
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
            // All of the literals we use are ASCII which means any bytes in the buffer that we
            // have successfully matched were single-bye UTF-8 characters. As a result, the first
            // byte that doesn't match is not only the nth byte in the buffer but the first byte of
            // the nth UTF-8 character. This means it's safe to get the nth character even if that
            // character is multi-byte.
            //
            // ... unless we haven't read the entire character :/
            //
            // todo: Fix this
            return Err(Error::JsonError(format!(
                "unexpected byte '{}'",
                self.buf[self.read_pos]
            )));
        }
        Ok(token)
    }

    fn scan_to_content(&mut self) -> Result<bool, Error> {
        self.discard_whitespace();

        while self.buffered_unread().len() == 0 {
            // All of the data in the buffer has been consumed by returned tokens. This is most
            // likely to occur on the first call to next or after the whole stream has been parsed
            // successfully, but it could also mean that a read into the buffer just happened to
            // align with a token boundary. Either way we should attempt a read to the front of the
            // buffer to get as much data as we can without allocating.
            match self.read_to_front_of_buffer() {
                // Nothing left to read means we parsed everything successfully.
                Ok(0) => return Ok(false),
                Ok(n) => self.write_pos += n,
                Err(err) => return Err(Error::IoError(err)),
            }

            self.discard_whitespace();
        }

        Ok(true)
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
