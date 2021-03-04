extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct LogRecord {
    level: LogLevel,
    timestamp: DateTime<Utc>,
    category: String,
    message: String,
}

impl LogRecord {
    fn debug(cat: &str, msg: &str) -> Self {
        Self {
            level: LogLevel::Debug,
            timestamp: Utc::now(),
            category: cat.to_string(),
            message: msg.to_string(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rmp;
    use rmps::{Deserializer, Serializer};
    use std::io::Cursor;

    #[test]
    fn usage_rmp_basic() {
        let mut buf = Vec::new();
        let expect = true;
        rmp::encode::write_bool(&mut buf, expect).unwrap();
        assert_eq!([0xc3], buf[..]);
        assert_eq!(expect, rmp::decode::read_bool(&mut &buf[..]).unwrap());
    }
    #[test]
    fn usage_rmp_struct() {
        let value = LogRecord::debug("test.dummy", "hellow world");
        let buf = rmp_serde::to_vec(&value).unwrap();
        let cur = Cursor::new(&buf[..]);
        let mut de = Deserializer::new(cur);
        let actual: LogRecord = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(&value, &actual);
    }
}
