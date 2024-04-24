#[derive(Debug)]
enum KeyType{
    Single(String),
    Multiple(Vec<String>)
}
#[derive(Debug)]
struct Data{
    name: String,
    key: KeyType
}

fn main() {
    let value = Data{
        name: "Roger".to_string(),
        key: KeyType::Multiple(vec!["1".to_string(),"2".to_string(),"3".to_string()])
    };

    //Test if key is vec or unique data
    match value.key {
        KeyType::Single(s) => {
            //logical for string here
            println!("{}",s)
        },
        KeyType::Multiple(vec) => {
            //logical for vec here
            println!("{:?}",vec)
        }
    };


}
