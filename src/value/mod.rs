use std::io;
use std::io::prelude::*;
use std::io::Write;

pub enum JsonValue {
    Object(String),
    String(String),
    Number(i32),
    Boolean(bool),
    Array(Vec<JsonValue>),
    Null,
}

pub trait Generator {
    type T: Write;
    fn get_writer(&mut self) -> &mut Self::T;

    fn write(&mut self, slice: &[u8]) -> io::Result<()> {
        self.get_writer().write_all(slice);
        Ok(())
    }
    //fn write_string(&self, slice: &[u8]) -> Result<()> {}

    //fn write_number(&self, slice: &i32) -> Result<()> {}

    fn write_json(&mut self, json: &JsonValue) -> io::Result<()> {
        match *json {
            JsonValue::Null => self.write(b"null"),
            JsonValue::Number(ref i32) => self.write(b"null"),
            JsonValue::Boolean(true) => self.write(b"true"),
            JsonValue::Boolean(false) => self.write(b"false"),
            JsonValue::String(ref String) => self.write(b"null"),
            JsonValue::Object(ref String) => self.write(b"null"),
            JsonValue::Array(ref array) => self.write(b"null"),
        }
    }
    // add code here
}

struct DumpGenerator{
    content: Vec<u8>,
}

impl DumpGenerator{
    fn new() -> Self{
        DumpGenerator{ 
            content: Vec::with_capacity(1024),
        }
    }
    pub fn consume(self) -> String {
        // Original strings were unicode, numbers are all ASCII,
        // therefore this is safe.
        unsafe { String::from_utf8_unchecked(self.content) }
    }

    //fn consue(self) -> String{}
}

impl Generator for DumpGenerator {
    // add code here
    type T = Vec<u8>;
    fn get_writer(&mut self) -> &mut Vec<u8> {
        &mut self.content
    }
}

impl JsonValue {
    /*
    pub fn new_arrary() -> JsonValue{
        JsonValue::Array(Vec::new())
    }
    
    pub fn new_object() -> JsonValue{
        JsonValue::Object(Object::new())
        }
    */

    pub fn dump(&self) -> String{
        let mut jsonWrite = DumpGenerator::new();
        jsonWrite.write_json(self).expect("test");
        jsonWrite.consume()
    }
}

/*
impl Serializaton for JsonValue {
    fn Serializaton() {
        if condition {
            unimplemented!();
        }else {
            match expr {
                Some(expr) => expr,
                None => expr,
            }
        }
    }
}

impl Deserialization for JsonValue {

}
*/