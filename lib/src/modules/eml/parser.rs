use std::mem;

use crate::modules::protos::eml::{Eml, Header};
use base64::prelude::*;
use bstr::ByteSlice;
use indexmap::IndexMap;
use nom::Err;

// type NomError<'a> = nom::error::Error<&'a [u8]>;

/// An Eml parser
pub struct EmlParser {
    result: Eml,
}

// the anatomy of an email can be defined like so:
// headers (check for content-type)
// body
//
impl EmlParser {
    /// Creates a new parser for Eml files
    pub fn new() -> Self {
        Self { result: Eml::default() }
    }

    pub fn parse<'a>(
        &mut self,
        input: &'a [u8],
    ) -> Result<Eml, Err<nom::error::Error<&'a [u8]>>> {
        self.result.is_eml = Some(true);
        let boundary = input.find("\n\n").or_else(|| input.find("\r\n\r\n"));

        let (header_part, body) = match boundary {
            Some(pos) => {
                let offset = if input[pos] == b'\r' { 4 } else { 2 };
                (&input[..pos], &input[pos + offset..])
            }
            None => (input, &[][..]),
        };

        let mut last_key: Option<&[u8]> = None;
        let mut headers = IndexMap::<&[u8], Vec<u8>>::new();

        for line in header_part.lines() {
            if line.starts_with(b" ") || line.starts_with(b"\t") {
                // multiline value
                // we need to append to the previous value here
                if let Some(last_key) = last_key {
                    if let Some(value) = headers.get_mut(last_key) {
                        value.push(b' ');
                        value.extend_from_slice(line.trim());
                    }
                }
            } else if let Some((key, value)) = line.split_once_str(":") {
                // new key:value pair
                let key_trimmed = key.trim();

                headers.insert(key_trimmed, value.trim().to_vec());
                last_key = Some(key_trimmed);
            }
        }

        for (key, value) in &headers {
            self.result.headers.push(Header {
                key: Some(key.to_vec()),
                value: Some(value.to_vec()),
                ..Default::default()
            });
        }

        // we parsed the initial headers, now we need take a look at the body/boundaries
        // Content-Type should be checked for multipart and boundary
        if let Some(content_type) = headers.get(&b"Content-Type"[..]) {
            // check if starts with multipart/
            if content_type.starts_with(b"multipart/") {
                if let Some(pos) = content_type.find("boundary=") {
                    // if so, find pos of boundary=
                    let start = pos + "boundary=".len();
                    let remainder = content_type[start..].trim();

                    // value could be "value" or value
                    // deal with that here
                }
            }
        }

        self.result.body = Some(body.trim().to_vec());

        // check if headers contain Content-Transfer-Encoding
        if let Some(content_encoding) =
            headers.get(&b"Content-Transfer-Encoding"[..])
        {
            if content_encoding == &b"base64"[..] {
                let cleaned_body: Vec<u8> = body
                    .iter()
                    .filter(|&&b| !b.is_ascii_whitespace())
                    .cloned()
                    .collect();

                let decoded_content =
                    BASE64_STANDARD.decode(cleaned_body).unwrap();
                self.result.decoded_body = Some(decoded_content);
            }
        }

        Ok(mem::take(&mut self.result))
    }
}
