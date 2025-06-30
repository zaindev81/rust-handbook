pub fn result_main() {
    let some_value: Option<i32> = Some(10);
    let none_value: Option<i32> = None;

    if some_value.is_some() {
        println!("Some value: {}", some_value.unwrap());
    } else {
        println!("No value present");
    }

    if none_value.is_none() {
        println!("No value present");
    } else {
        println!("Some value: {}", none_value.unwrap());
    }

    let name: Option<String> = get_user_name();

    if name.is_some() {
        println!("User name: {}", name.unwrap());
    } else {
        println!("No user name found");
    }
}

fn get_user_name() -> Option<String> {
    Some("Alice".to_string())
}
