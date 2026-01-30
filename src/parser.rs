use std::fmt::Display;

use serde::{Deserialize, forward_to_deserialize_any};

pub struct CardDeserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de str,
}

#[derive(Debug)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case JSON.
    Eof,
    Syntax,
    TrailingCharacters,
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::Syntax => formatter.write_str("Syntax Error"),
            Error::TrailingCharacters => formatter.write_str("trailing input"), /* and so forth */
        }
    }
}

impl std::error::Error for Error {}

impl<'de> CardDeserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_str(input: &'de str) -> Self {
        CardDeserializer { input }
    }
}
pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = CardDeserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de> CardDeserializer<'de> {
    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char, Error> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    // Consume the first character in the input.
    fn eat_char(&mut self) {
        match self.peek_char() {
            Ok(ch) => {
                self.input = &self.input[ch.len_utf8()..];
            }
            Err(er) => {}
        };
    }

    fn parse_str(&mut self) -> Result<&str, Error> {
        match self.input.find('\n') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                Ok(s)
            }
            None => {
                if !self.input.is_empty() {
                    let s = &self.input;
                    Ok(s)
                } else {
                    Err(Error::Eof)
                }
            }
        }
    }

    fn parse_u16(&mut self) -> Result<u16, Error> {
        let mut int = 0;
        loop {
            match self.input.chars().next() {
                Some(ch @ '0'..='9') => {
                    self.input = &self.input[1..];
                    int *= u16::from(10u8);
                    int += u16::from(ch as u8 - b'0');
                }
                _ => {
                    self.eat_char();
                    self.eat_char();
                    return Ok(int);
                }
            }
        }
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut CardDeserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_char()? {
            '0'..='9' => self.deserialize_u16(visitor),
            'A'..='Z' => self.deserialize_str(visitor),
            'a'..='z' => self.deserialize_str(visitor),
            _ => Err(Error::Syntax),
        }
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(self.parse_str()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(&mut *self)
    }
    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u32 u64 u128 f32 f64 char string
        bytes byte_buf option unit unit_struct map newtype_struct tuple
        tuple_struct enum identifier ignored_any
    }
}

impl<'de> serde::de::SeqAccess<'de> for CardDeserializer<'de> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if !self.input.is_empty() {
            seed.deserialize(self).map(Some)
        } else {
            Ok(None)
        }
    }
}
