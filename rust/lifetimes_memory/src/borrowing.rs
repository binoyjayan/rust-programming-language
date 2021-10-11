
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn borrowing() {
    // Instead of taking ownership of a vector, borrow it
    let v1 = vec![1, 2, 3];
    let print_vector = |v:&Vec<i32>| {
        println!("{:?}", v);
    };
    print_vector(&v1);
    println!("{:?}", v1);


    // Mutable reference (primitive types)
    let mut a = 10;
    let b = &mut a;
    *b += 5;

    // Mutable reference (objects)
    let mut v2 = vec![10, 20, 30];
    let print_vector = |v:&Vec<i32>| {
        println!("{:?}", v);
    };
    let v3 = &mut v2;
    v3.push(40);
    print_vector(&v2);
    for e in &v2 {
        println!("element = {}", e);
    }
}
