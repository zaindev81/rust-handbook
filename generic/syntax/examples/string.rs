// | Type         | Owned / Borrowed | Mutable | Stored on | Usage                               |
// | ------------ | ---------------- | ------- | --------- | ----------------------------------- |
// | **`String`** | Owned            | Yes     | Heap      | Growable, UTF-8 encoded string.     |
// | **`&str`**   | Borrowed slice   | No      | Anywhere  | Read-only view into a UTF-8 string. |

fn main() {
    string_operation();
    str_operation();
    creating_operation();
    modifying_operation();
}

fn basic_operation() {
    let owned: String = String::from("Hello"); // String
    let borrowed: &str = "Hello";              // &str（literal)
    let slice: &str = &owned[0..2];            // &str（part of String)
}

// Heap-allocated, growable.
// Used when you need to own the string and possibly modify it.
// - When you need to create or modify a string
// - When you need to return it from a function (and transfer ownership)
// - When you need a string that is stored on the heap for long-term use
fn string_operation() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
}

// A view into a string, not an owner.
// Often points to string literals ("Hello") or part of another string.
// - When you only need to read a string without modifying it
// - When you want a function parameter to accept both String and string literals
// - When you want to take a partial reference (slice) of a string
fn str_operation() {
    let s: &str = "Hello, world!";
    println!("{}", s);

    let hello = "Hello world";
    let hello_slice: &str = &hello[0..5]; // Slicing a string
    println!("{}", hello_slice);
}

fn creating_operation() {
    // From literal
    let s1 = String::from("Hello");

    // Using to_string()
    let s2 = "Hello".to_string();

    // Empty
    let s3 = String::new();

    // From another string
    let s4 = s1.clone(); // clone the data
}

fn modifying_operation() {
    let mut s = String::from("Hi");
    s.push('!');              // push a char
    s.push_str(" there");     // push a &str
    s.insert(0, 'H');         // insert at index
    s.replace("Hi", "Hello"); // replace substring
    println!("Modified string: {}", s);
}

fn concatenation_operation() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + " " + &s2; // s1 is moved here
    println!("{}", s3);

    let s = format!("{} {}", "Hello", "World"); // s1, s2 not moved
}