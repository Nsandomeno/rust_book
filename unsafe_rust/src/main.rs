use std::slice;

static HELLO_WORLD: &str = "Hello World!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    
    println!("{HELLO_WORLD}");
    raw_pt_ex();
    unsafe {
        println!("{COUNTER}");
        unsafe_fn();
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// unsafe super powers
// 1 deference a raw pointer
fn raw_pt_ex() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1); // if not in an unsafe block could the data being pointed to could not be referenced
        println!("r2 is: {}", *r2);
    }
}
// 2 call an unsafe function or method
unsafe fn unsafe_fn() {}
fn safe_wrapper_for_splitting(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!")
}

// 3 access or modify a mutable static variable (SEE TOP OF FILE)
// 4 implement an unsafe trait
unsafe trait Foo {}
unsafe impl Foo for i32 {}
// 5 access fields of union 's 

