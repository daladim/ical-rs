//! ical-daladim
//!
//! This is a fork of the [ical](https://docs.rs/ical/latest/ical/) crate, because I needed to tag a version on a specific commit.
//!
//! You most likely won't find anything interesting, please use the upstream version instead.
//!
//!
#[macro_use]
extern crate thiserror;

const PARAM_VALUE_DELIMITER: char = ',';
const VALUE_DELIMITER: char = ':';
const PARAM_DELIMITER: char = ';';
const PARAM_NAME_DELIMITER: char = '=';
const PARAM_QUOTE: char = '"';

#[cfg(any(feature = "ical", feature = "vcard"))]
pub mod parser;

#[cfg(feature = "ical")]
pub use crate::parser::ical::IcalParser;

#[cfg(feature = "vcard")]
pub use crate::parser::vcard::VcardParser;

#[cfg(feature = "property")]
pub mod property;
#[cfg(feature = "property")]
pub use crate::property::PropertyParser;

#[cfg(feature = "line")]
pub mod line;
#[cfg(feature = "line")]
pub use crate::line::LineReader;
