use std::collections::HashMap;

fn main() {

    let mut map = HashMap::new();

    map.insert(0, "Aus");
    map.insert(1, "NZ");
    map.insert(2, "SG");

    for (key, val) in map.iter() {
        println!("key: {} val: {}", key, val);
    }

}

