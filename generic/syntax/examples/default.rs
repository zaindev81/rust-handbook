#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
        }
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: "Unknown".to_string(),
            age: 0,
        }
    }
}

fn main() {
    let origin = Point::default();
    println!("Origin point: ({}, {})", origin.x, origin.y);

    let config = Config::default();
    println!("Config: host={}, port={}, debug={}", config.host, config.port, config.debug);

    let person = Person::default();
    println!("Person: name={}, age={}", person.name, person.age);
}