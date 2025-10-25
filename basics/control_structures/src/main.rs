fn main() {
    if_else();
    matches();
    loops();
    some_none_loops();
}

// if else
fn if_else() {
    let number = 42;
    let flag = false;

    if ((number > 50) && (!flag)) {
        println!("(number > 50) && (!flag) ");
    } else if (number <= 50) && (flag) {
        println!("(number <= 50) && (flag)");
    } else {
        println!("else");
    }
}

// matches - switch case
fn matches() {
    let number = 3;
    match number {
        1 => println!("Eins"),
        2 => println!("Zwei"),
        3 => println!("Drei"),
        _ => println!("Etwas anderes"), // _ = default case
    }

    // match with multiple values
    match number {
        1 | 2 => println!(" 1 | 2"),
        3 | 4 | 5 => println!("v"),
        6..=10 => println!("beween 6 and 10"), // Range
        _ => println!("default case"),
    }

    // match with return value
    let description = match number {
        1..=3 => "low",
        4..=6 => "average",
        7..=10 => "high",
        _ => "unknown",
    };
}

// loops
fn loops() {
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 10; // return value of this loop
        }
    };

    // while
    let mut countdown = 3;
    print!("Countdown: ");
    while countdown > 0 {
        print!("{} ", countdown);
        countdown -= 1;
    }

    // for
    print!("For 1-5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // for mit Array/Vec
    let fruits = ["Apfel", "Banane", "Orange"];
    for fruit in fruits {
        println!("  - {}", fruit);
    }

    // for mit Index
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}. {}", index + 1, fruit);
    }

    // Nested loops mit labels
    'outer: for x in 1..=3 {
        for y in 1..=3 {
            if x == 2 && y == 2 {
                println!("Breaking outer loop at ({}, {})", x, y);
                break 'outer; // Bricht die äußere Schleife ab
            }
            print!("({},{}) ", x, y);
        }
    }
    println!();

    println!();
}

// Some and None loops
fn some_none_loops() {
    // while let - iteration over option/result
    let mut optional_values = vec![Some(1), Some(2), None, Some(3)]; // Some and None is important for containers, end = None
    while let Some(opt) = optional_values.pop() {
        // delete values until reaching None
        if let Some(value) = opt {
            println!("while let: {}", value);
        }
    }
}
