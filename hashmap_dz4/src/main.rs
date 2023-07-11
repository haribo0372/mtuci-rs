use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read};

#[derive(Debug)]
struct CategoryFood{
    category: String
}
fn main() {
    let path = "food.txt";
    let mut file = File::open(path);
    let mut file = match file {
        Ok(a) => a,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create(path).expect("Error creating file"),
            other => panic!("ERROR")
        }
    };
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap();
    println!("{:#?}", build_hashmap(file_data))
}

fn build_hashmap(data: String) -> HashMap<String, CategoryFood>{
    let mut hm_food: HashMap<String, CategoryFood> = HashMap::new();
    let mut vec = Vec::new();
    for r in data.split(','){vec.push(r)}
    hm_food.entry(vec[0].to_string()).or_insert(
        CategoryFood{category: vec[1].to_string()});
    for i in 1..vec.len()-2{
        hm_food.entry(vec[i+1].to_string()).or_insert(CategoryFood{category: vec[i+2].to_string()}) ;
    }
    let category = vec!["Fruit", "Meat", "Vegetable"];
    for cat in category{
        match hm_food.entry(cat.to_string()){
            Entry::Occupied(a) => {hm_food.remove(cat);}
            Entry::Vacant(_) => {}
    }}
    hm_food.insert("Veal".to_string(), CategoryFood{category: "Meat".to_string()});

    hm_food
}
