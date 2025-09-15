enum Filter {
    Blur,
    Brighten,
    Crop,
    Rotate,
}

// This converts a Filter enum into a static string (&'static str).
// The From trait allows automatic conversion using .into() or From::from().
// The match statement maps each enum variant to its string equivalent.
impl From<Filter> for &'static str {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Blur => "blur",
            Filter::Brighten => "brighten",
            Filter::Crop => "crop",
            Filter::Rotate => "rotate",
        }
    }
}

fn main() {
    let filter = Filter::Blur;
    let filter_str: &'static str = filter.into();
    println!("Filter as string: {}", filter_str);

    let f = Filter::Crop;
    let s: &'static str = f.into();
    println!("Filter as string: {}", s);
}
