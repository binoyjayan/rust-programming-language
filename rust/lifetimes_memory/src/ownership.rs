
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn ownership() {
    let v1 = vec![1, 2, 3];
    // moves a pointer to the vector from v1 to v2
    // Now v2 owns the vector. v1 is now invalidated
    let v2 = v1;

    // It works on primitive data types as they are copied
    let u1 = 1;
    let u2 = u1;
    println!("u1 = {}", u1);

    // But it will not work if they are Box-ed
    let b1 = Box::new(10);
    let b2 = b1;
    // println!("u1 = {}", b1); // This will not compile

    // Take ownership of a vector and then return the ownership back to caller
    let v3 = vec![10, 20, 30];
    let print_vector = |v:Vec<i32>| -> Vec<i32> {
        println!("{:?}", v);
        v
    };
    let v4 = print_vector(v3);
    println!("{:?}", v4);
}