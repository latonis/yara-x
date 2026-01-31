/*! YARA module that parses EML files.

An EML file is the standard file format for email, according to RFC 2045 [1]

[1]: https://datatracker.ietf.org/doc/html/rfc2045
 */

use crate::modules::prelude::*;
use crate::modules::protos::eml::*;
pub mod parser;

#[module_main]
fn main(data: &[u8], _meta: Option<&[u8]>) -> Result<Eml, ModuleError> {
    match parser::EmlParser::new().parse(data) {
        Ok(eml) => Ok(eml),
        Err(_) => {
            let mut eml = Eml::new();
            eml.is_eml = Some(false);
            Ok(eml)
        }
    }
}
