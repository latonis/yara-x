use std::{collections::HashMap, mem};

use crate::modules::protos::eml::{Eml, EmlPart, Header};
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

        // stack for processing
        let mut stack: Vec<&[u8]> = vec![input];
        let mut is_root = true;
        // keep track to prevent mime bomb shenanigans
        let mut parts_processed = 0;

        while let Some(current_data) = stack.pop() {
            parts_processed += 1;
            if parts_processed > 100 {
                break;
            }

            let (header, body) = self.split_message(current_data);
            let headers = self.parse_headers(header);

            if is_root {
                self.result.headers = self.map_to_proto_headers(&headers);
                self.result.body = Some(body.to_vec());
                self.result.decoded_body = self.decode_body(&headers, body);
            }

            // Content-Type should be checked for multipart and boundary
            let mut found_boundary = None;
            if let Some(content_type) = headers.get(&b"Content-Type"[..]) {
                // check if starts with multipart/
                if content_type.starts_with(b"multipart/") {
                    if let Some(pos) = content_type.find("boundary=") {
                        // if so, find pos of boundary=
                        let start = pos + "boundary=".len();
                        let remainder = content_type[start..].trim();

                        // value could be "value" or value
                        let boundary_bytes = if remainder.starts_with(b"\"") {
                            // enclosed by ""
                            remainder.split_str("\"").nth(1).unwrap_or(&[])
                        } else {
                            // no quotes but may be multiple items in here, split and grab 0th
                            remainder
                                .split_str(";")
                                .next()
                                .unwrap_or(&[])
                                .trim()
                        };

                        if !boundary_bytes.is_empty() {
                            let mut delimiter = b"--".to_vec();
                            delimiter.extend_from_slice(boundary_bytes);
                            found_boundary = Some(delimiter);
                        }
                    }
                }
            }

            if let Some(delimiter) = found_boundary {
                let parts: Vec<&[u8]> = body.split_str(&delimiter).collect();
                for part in parts.into_iter().rev() {
                    let trimmed = part.trim();

                    if trimmed.is_empty() || trimmed.starts_with(b"--") {
                        continue;
                    }
                    stack.push(trimmed);
                }
            } else if !is_root {
                self.result.parts.push(EmlPart {
                    headers: self.map_to_proto_headers(&headers),
                    body: Some(body.to_vec()),
                    decoded_body: self.decode_body(&headers, body),
                    ..Default::default()
                });
            }
            is_root = false;
        }

        Ok(mem::take(&mut self.result))
    }

    fn split_message<'a>(&self, input: &'a [u8]) -> (&'a [u8], &'a [u8]) {
        if let Some(pos) = input.find("\r\n\r\n") {
            (&input[..pos], &input[pos + 4..])
        } else if let Some(pos) = input.find("\n\n") {
            (&input[..pos], &input[pos + 2..])
        } else {
            (input, &[][..])
        }
    }

    fn decode_body(
        &self,
        headers: &IndexMap<&[u8], Vec<u8>>,
        body: &[u8],
    ) -> Option<Vec<u8>> {
        let content_encoding =
            headers.get(&b"Content-Transfer-Encoding"[..])?;

        if content_encoding == &b"base64"[..] {
            let cleaned_body: Vec<u8> = body
                .iter()
                .filter(|&&b| !b.is_ascii_whitespace())
                .cloned()
                .collect();

            return BASE64_STANDARD.decode(cleaned_body).ok();
        }

        if content_encoding == &b"Quoted-Printable"[..] {
            return quoted_printable::decode(
                body,
                quoted_printable::ParseMode::Robust,
            )
            .ok();
        }
        None
    }

    fn map_to_proto_headers(
        &self,
        headers: &IndexMap<&[u8], Vec<u8>>,
    ) -> Vec<Header> {
        headers
            .iter()
            .map(|(k, v)| Header {
                key: Some(k.to_vec()),
                value: Some(v.to_vec()),
                ..Default::default()
            })
            .collect()
    }

    fn parse_headers<'a>(
        &self,
        headers_raw: &'a [u8],
    ) -> IndexMap<&'a [u8], Vec<u8>> {
        let mut last_key: Option<&[u8]> = None;
        let mut headers = IndexMap::<&[u8], Vec<u8>>::new();

        for line in headers_raw.lines() {
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

        headers
    }
}
