fn main() {
    copy_vs_reference();
    return_types();
    closure();
}

// copy vs reference
fn copy_vs_reference() {
    // copy by value -> primitive types
    let x = 42;
    let result = copy_by_value(x);

    // copy by reference -> complex types (with heap)
    let name = String::from("Rust");
    let length = copy_by_reference(&name);

    // move semantics -> complex types (with heap)
    let text = String::from("Hello");
    let moved = take_ownership(text);
}

fn copy_by_value(num: i32) -> i32 {
    return num * 2; // num * 2 is also possible without semicolon -> return value
}
fn copy_by_reference(text: &String) -> usize {
    return text.len();
}
fn take_ownership(text: String) -> String {
    return text.to_uppercase();
}

// return types
fn return_types() {
    let sum = add(5, 3);
    println!("add(5, 3) = {}", sum);

    // options (None oder Some)
    let ret_value = safe_divide(10.0, 2.0);
    if ret_value.is_some() {
        println!("is_some(): Wert vorhanden = {}", ret_value.unwrap());
    }
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;
    } else {
        return Some(a / b);
    }
}

// closure
fn closure() {
    let add_ten = |x: i32, y: u32| x + (y as i32) + 10;
    println!("Closure: {}", add_ten(5, 7));

    // closure mit captured variable
    let multiplier = 3;
    let multiply = |x| x * multiplier;
    println!("Captured: {}", multiply(5));

    println!();
}
