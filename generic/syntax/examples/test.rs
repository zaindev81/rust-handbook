fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to __!"
}

fn compute(a: u32, b: u32) -> u32 {
    // Don't touch the function body.
    a + b * 2
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use crate::greeting;
    use crate::compute;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }

    #[test]
    fn case() {
        assert_eq!(compute(2, 3), 8);
    }
}