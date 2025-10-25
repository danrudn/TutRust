fn main() {
    options();
    error_results();
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
fn custom_errors() {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
        InvalidInput(String),
    }

    // define how the error is display via println: println({}, MathError)
    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
                MathError::NegativeSquareRoot => {
                    write!(f, "Cannot take square root of negative number")
                }
                MathError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            }
        }
    }

    impl std::error::Error for MathError {}

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

    // Complex operation with multiple error sources
    fn complex_calculation(a: f64, b: f64) -> Result<f64, MathError> {
        let division = safe_divide(a, b)?;
        let sqrt_result = safe_sqrt(division)?;
        Ok(sqrt_result)
    }

    // Testen der Custom Errors
    match complex_calculation(16.0, 4.0) {
        Ok(result) => println!("Complex calc result: {}", result),
        Err(e) => println!("Complex calc error: {}", e),
    }

    match complex_calculation(16.0, 0.0) {
        Ok(result) => println!("Complex calc result: {}", result),
        Err(e) => println!("Complex calc error: {}", e),
    }

    match complex_calculation(-16.0, 4.0) {
        Ok(result) => println!("Complex calc result: {}", result),
        Err(e) => println!("Complex calc error: {}", e),
    }

    println!();
}
