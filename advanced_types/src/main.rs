fn main() {
    test_alias();
    let f: Thunk = Box::new(|| println!("hi"));


}

// type alias
type Result<T> = std::result::Result<T, std::io::Error>; // commonly used in Rust!
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn test_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
    Box::new(|| ())
}

