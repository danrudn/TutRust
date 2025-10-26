fn main() {
    options();
    error_results();
    custom_errors();
}

// options
fn options() {
    let numbers = vec![1, 2, 3, 4, 5];

    // find() returns Option<&T> (None or Some(<T>))
    let found = numbers.iter().find(|&&x| x == 3);
    match found {
        Some(value) => println!("Found: {}", value),
        None => println!("Not Found"),
    }

    // option1: check if maybe_number is Some
    let maybe_number: Option<i32> = Some(42);
    match maybe_number {
        Some(n) => println!("Number: {}", n),
        None => println!("Kein Wert"),
    }

    // option2: check if maybe_number is Some
    if let Some(n) = maybe_number {
        println!("if let: {}", n);
    }
    println!();
}

// error results
fn error_results() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("Ergebnis: {}", result),
        Err(e) => println!("Fehler: {}", e),
    }
}
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division durch Null".to_string())
    } else {
        Ok(a / b)
    }
}

// own error types
// - define custom Error enum
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput(String),
}

// - implement display trait for user-friendly output
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division durch Null nicht erlaubt"),
            MathError::NegativeSquareRoot => {
                write!(f, "Wurzel aus negativer Zahl nicht möglich")
            }
            MathError::InvalidInput(msg) => write!(f, "Ungültige Eingabe: {}", msg),
        }
    }
}

// - implement error trait
impl std::error::Error for MathError {}

// throw own error type
fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn custom_errors() {
    // example 1: successful operation
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Fehler: {}", e),
    }

    // example 2: division by zero - specific error matching
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("Ergebnis: {}", result),
        Err(MathError::DivisionByZero) => {
            println!("Special handling: Division by zero detected!");
        }
        Err(MathError::NegativeSquareRoot) => {
            println!("Special handling: Negative square root!");
        }
        Err(MathError::InvalidInput(msg)) => {
            println!("Special handling: Invalid input - {}", msg);
        }
    }

    // example 3: negative square root - specific error matching
    match safe_sqrt(-4.0) {
        Ok(result) => println!("Wurzel: {}", result),
        Err(MathError::NegativeSquareRoot) => {
            println!("Special handling: Cannot take square root of negative number!");
        }
        Err(other_error) => println!("Anderer Fehler: {}", other_error),
    }

    // example 4: match error variants individually
    let error = MathError::InvalidInput("Test".to_string());
    match error {
        MathError::DivisionByZero => println!("Error type: Division by zero"),
        MathError::NegativeSquareRoot => println!("Error type: Negative square root"),
        MathError::InvalidInput(details) => println!("Error type: Invalid input ({})", details),
    }

    println!();
}
