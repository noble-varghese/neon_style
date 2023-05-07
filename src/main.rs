use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert("a", "1");
    map.insert("b", true);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
