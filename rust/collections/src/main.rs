#[warn(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("Standard Collections");
    // vectors();
    // hash_map();
    // hash_set();
    iterators();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn vectors() {
    let mut a = Vec::new();
    a.push(111);
    a.push(222);
    a.push(333);
    a.push(444);
    a.push(555);
    println!("vector = {:?}", a);

    let i = 5;
    // Use 'get' to do vector boundary check
    match a.get(i) {
        Some(x) => println!("a[{}] = {}", i, x),
        None => println!("No element at index {}", i),
    }

    // iterate
    for x in &a {
        println!("{}", x);
    }

    // pop one element
    let res = a.pop();
    match res {
        Some(x) => println!("popped last element {}", x),
        None => println!("No element to pop"),
    }
    println!("vector = {:?}", a);

    // pop all elements
    while let Some(x) = a.pop() {
        println!("popped {}", x);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn hash_map() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("A square has {} sides", shapes["square".into()]);

    shapes.insert("square".into(), 5);

    for (key, val) in &shapes {
        println!("{} : {}", key, val);
    }
    println!("shapes = {:?}", shapes);

    // insert only if not present
    shapes.entry("circle".into()).or_insert(1);
    println!("shapes = {:?}", shapes);
    {
        // let actual = shapes.entry("circle".into()).or_insert(2);
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0; // reference to the entry for 'circle'
    }
    println!("shapes = {:?}", shapes);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn hash_set() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("delta");
    println!("set = {:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("added vega");
    }
    println!("set = {:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("added vega");
    }
    println!("set = {:?}", greeks);

    if !greeks.contains("kappa") {
        println!("doesn't contain kappa");
    }
    if greeks.remove("delta"){
        println!("removed kappa");
    }

    // random sets
    let set_01_05: HashSet<_> = (1..=5).collect();
    let set_06_10: HashSet<_> = (6..=10).collect();
    let set_01_10: HashSet<_> = (1..=10).collect();
    let set_02_08: HashSet<_> = (2..=8).collect();

    //subset
    println!("is {:?} a subset of {:?} - {}",
             set_02_08, set_01_10, set_02_08.is_subset(&set_01_10));

    //disjoint
    println!("is {:?} and {:?} are disjoint - {}",
             set_01_05, set_06_10, set_01_05.is_disjoint(&set_06_10));

    //intersection
    println!("union of {:?} and {:?} is - {:?}",
             set_02_08, set_06_10, set_02_08.union(&set_06_10));

    //union
    println!("intersection of {:?} and {:?} is - {:?}",
             set_02_08, set_06_10, set_02_08.intersection(&set_06_10));

    // difference = union - intersection
    println!("difference of {:?} and {:?} is - {:?}",
             set_02_08, set_06_10, set_02_08.difference(&set_06_10));
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn iterators() {
    let vec = vec![1, 2, 3];
    // always borrow from vec in iterators or use 'iter'
    println!("Vector contents:");
    for x in &vec {
        // note the deference operator
        println!("{}", *x);
    }

    println!("Vector contents using iter:");
    for x in vec.iter() {
        // dereference operator is not mandatory. compiler figures it
        println!("{}", x);
    }

    // Mutable iterators
    println!("Vector contents using iter_mut:");
    let mut vec_mut = vec![3, 2, 1];
    for x in vec_mut.iter_mut() {
        *x += 2;
         println!("{}", x);
    }

    println!("Vector in reverse");
    for x in vec.iter().rev() {
        println!("{}", x);
    }

    // into iter (move operation)
    let mut vec2 = vec![3, 2, 1];
    // extend method uses into_iter to move elements
    // out of vector to an iterator
    // let it  = vec.into_iter();
    // cannot use vec anymore
    vec2.extend(vec);
    println!("{:?}", vec2);
}

