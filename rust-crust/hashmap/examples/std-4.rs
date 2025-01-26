use hashmap::HashMap;

fn main() {
    let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter()
        .cloned()
        .collect();

    for (country, resources) in &timber_resources {
        println!("{} has {} timber resources", country, resources);
    }
}
