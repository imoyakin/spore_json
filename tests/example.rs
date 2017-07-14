extern crate json;

//pub mod read;

fn main(){
    println!("Hello, world!");
    //let mut fileadress = String::from("C:/Users/reiko/Documents/catalog.json");
    let mut data = String!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true]
    };

    data.dump();

    println!("{:#}", data);
    //json::read::ReadJson(&fileadress);
}