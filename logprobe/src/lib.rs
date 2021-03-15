extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
use std::collections::HashMap;



use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Value {
    Null,
    String(String),
    Bool(bool),
    Number(Number),
    Object(Object),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Number {
    I64(i64),
    U64(u64),
    F64(f64),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Object {
    name: String,
    map: HashMap<String, Value>,
}

pub trait Mapper {
    fn to_object(&self) -> Object;
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct ArgMap {
    map: HashMap<String, Value>,
}

impl ArgMap {
    fn new() -> Self {
        Self{
            map: HashMap::new(),
        }
    }
    fn string(&mut self, key: &str, value: &str){
        self.map.insert(key.to_string(), Value::String(value.to_string()));
    }
    fn bool(&mut self, key: &str, value: bool){
        self.map.insert(key.to_string(), Value::Bool(value));
    }
    fn f64(&mut self, key: &str, value: f64){
        self.map.insert(key.to_string(), Value::Number(Number::F64(value)));
    }
    fn i64(&mut self, key: &str, value: i64){
        self.map.insert(key.to_string(), Value::Number(Number::I64(value)));
    }
    fn u64(&mut self, key: &str, value: u64){
        self.map.insert(key.to_string(), Value::Number(Number::U64(value)));
    }
    fn object(&mut self, key: &str, value: Object){
        self.map.insert(key.to_string(), Value::Object(value));
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct LogRecord {
    level: LogLevel,
    timestamp: DateTime<Utc>,
    category: String,
    message: String,
    args: Option<ArgMap>,
}

impl LogRecord {
    fn debug(cat: &str, msg: &str) -> Self {
        Self {
            level: LogLevel::Debug,
            timestamp: Utc::now(),
            category: cat.to_string(),
            message: msg.to_string(),
            args: None,
        }
    }
    fn debugf(cat: &str, msg: &str, args: Option<ArgMap>) -> Self {
        Self {
            level: LogLevel::Debug,
            timestamp: Utc::now(),
            category: cat.to_string(),
            message: msg.to_string(),
            args: args,
        }
    }
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

        // Numbers
        let mut buf = Vec::new();
        let expect = 1u8;
        rmp::encode::write_u8(&mut buf, expect).unwrap();
        rmp::encode::write_u16(&mut buf, expect as u16).unwrap();
        rmp::encode::write_u32(&mut buf, expect as u32).unwrap();
        rmp::encode::write_u64(&mut buf, expect as u64).unwrap();
        rmp::encode::write_i8(&mut buf, expect as i8).unwrap();
        rmp::encode::write_i16(&mut buf, expect as i16).unwrap();
        rmp::encode::write_i32(&mut buf, expect as i32).unwrap();
        rmp::encode::write_i64(&mut buf, expect as i64).unwrap();

        let cur = Cursor::new(&buf[..]);
        let mut de = Deserializer::new(cur);
        assert_eq!(expect, Deserialize::deserialize(&mut de).unwrap());
        assert_eq!(expect as u16, Deserialize::deserialize(&mut de).unwrap());
        assert_eq!(expect as u32, Deserialize::deserialize(&mut de).unwrap());
        assert_eq!(expect as u64, Deserialize::deserialize(&mut de).unwrap());
        assert_eq!(expect as i8, Deserialize::deserialize(&mut de).unwrap());
        assert_eq!(expect as i16, Deserialize::deserialize(&mut de).unwrap());
        assert_eq!(expect as i32, Deserialize::deserialize(&mut de).unwrap());
        assert_eq!(expect as i64, Deserialize::deserialize(&mut de).unwrap());
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
    #[test]
    fn usage_rmp_struct_multi() {
        let mut buf = Vec::new();
        let value1 = LogRecord::debug("test.dummy", "hello world");
        let value2 = LogRecord::debug("test.dummy", "Are you like log?");
        value1.serialize(&mut Serializer::new(&mut buf)).unwrap();
        value2.serialize(&mut Serializer::new(&mut buf)).unwrap();

        let cur = Cursor::new(&buf[..]);
        let mut de = Deserializer::new(cur);
        let actual1: LogRecord = Deserialize::deserialize(&mut de).unwrap();
        let actual2: LogRecord = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(&value1, &actual1);
        assert_eq!(&value2, &actual2);
        assert_ne!(&value2, &actual1);
        assert_ne!(&value1, &actual2);
    }

    // #[test]
    // fn usage_rmp_parse_struct_from_file() {
    //     let mut file = File::open("testdata/dummy.msgpack").unwrap();
    //     let mut buf = Vec::new();
    //     file.read_to_end(&mut buf).unwrap();

    //     let cur = Cursor::new(&buf[..]);
    //     let mut de = Deserializer::new(cur);
    //     let actual1: LogRecord = Deserialize::deserialize(&mut de).unwrap();
    //     let actual2: LogRecord = Deserialize::deserialize(&mut de).unwrap();
    //     assert_eq!(&actual1.category, &actual2.category);
    //     assert_ne!(&actual1.message, &actual2.message);
    // }

    struct DummyStruct {
        a: i64,
        b: String,
    }
    impl Mapper for DummyStruct {
        fn to_object(&self) -> Object {
            let mut m = HashMap::new();
            m.insert("a".to_string(), Value::Number(Number::I64(self.a)));
            m.insert("b".to_string(), Value::String(self.b.to_string()));
            Object{
                name: "DummyStruct".to_string(),
                map: m,
            }
        }
    }

    #[test]
    fn log_debugf() {
        let mut args = ArgMap::new();
        args.string("name", "alty");
        args.bool("key_bool", true);
        args.i64("key_int", 32);
        args.u64("key_uint", 42);
        args.f64("key_uint", 42.195);
        args.object("key_object", DummyStruct{a:1, b:"test".to_string()}.to_object());
        let val = LogRecord::debugf("test.dummy", "Are you like log {name}", Some(args));

        print!("{:?}", val);
    }
}
