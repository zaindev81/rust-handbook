fn main() {
    basic_operations();
}

fn basic_operations() {
    println!("Basic Iter operations==================");
    let vec = vec![1, 2, 3, 4, 5];

    // Three ways to create iterators:

    // 1. iter() - borrows each element
    let iter1 = vec.iter();
    for item in iter1 {
        println!("Borrowed: {}", item); // item is &i32
    }

    // 2. into_iter() - takes ownership of each element
    let iter2 = vec.into_iter();
    for item in iter2 {
        println!("Owned: {}", item); // item is i32
    }

    // 3. iter_mut() - mutably borrows each element
    let mut vec_mut = vec![1, 2, 3, 4];
    let iter3 = vec_mut.iter_mut();
    for item in iter3 {
        *item *= 2;
        println!("Mutated: {}", item); // item is &mut i32
    }
}