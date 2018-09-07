mod value;
use value::JsonValue;

pub type Array = Vec<JsonValue>;


pub fn stringify<T>(start: T) -> String where T: Into<JsonValue>{
    let start: JsonValue = start.into();
    start.dump()
}

pub fn parse(){

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

