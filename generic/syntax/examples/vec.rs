fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.insert(0, 10);
    println!("v1: {:?}", v1);

    let mut v2: Vec<i32> = Vec::with_capacity(10);
    v2.extend([0, 1, 2]);
    println!("v2: {:?}", v2);

    let v3 = vec![1, 2, 3, 4, 5];
    let third = v3[2];
    println!("v3: {:?}", v3);
    println!("third {:?}", third);

    //
    // Accessing Elements
    //
    match v1.get(2) {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element"),
    }

    if let Some(first) = v1.first() {
        println!("First: {}", first);
    }

    if let Some(last) = v1.last() {
        println!("Last: {}", last);
    }

    let v4: Vec<i32> = (0..5).collect();
    println!("v4: {:?}", v4);

    //
    // Iterating
    //
    for item in &v1 {
        println!("{}", item);
    }

    for (i, item) in v1.iter().enumerate() {
        println!("{}: {}", i, item);
    }

    let mut v4 = vec![1, 2, 3];
    for item in &mut v4 {
        *item *= 2;
    }
    println!("v4: {:?}", v4);
}