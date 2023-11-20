use crate::hash_map::HashMap;

mod hash_map;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("1".to_string(), 1);
    map.insert("2".to_string(), 2);

    for x in &map {
        println!("{:?}", x);
    }

    println!("{:?}", map);
}
