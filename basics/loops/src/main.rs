fn main() {
    println!("=== Rust Loops: for, while, loop ===\n");

    for_loops();
    while_loops();
    loop_loops();
    nested_loops();
    loop_control();
}

// 1. for loops - most common
fn for_loops() {
    println!("1. For Loops");

    // basic range
    println!("Basic range (0..5):");
    for i in 0..5 {
        println!("  i = {}", i);
    }

    // inclusive range
    println!("Inclusive range (1..=5):");
    for i in 1..=5 {
        println!("  i = {}", i);
    }

    // iterate over array
    println!("Array iteration:");
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("  element = {}", element);
    }

    // iterate with index using enumerate
    println!("With index (enumerate):");
    let names = ["Alice", "Bob", "Charlie"];
    for (index, name) in names.iter().enumerate() {
        println!("  [{}] = {}", index, name);
    }

    // iterate over vector
    println!("Vector iteration:");
    let vec = vec![1, 2, 3, 4, 5];
    for value in &vec {
        // borrow to avoid moving
        println!("  value = {}", value);
    }

    // step by (every 2nd element)
    println!("Step by 2:");
    for i in (0..10).step_by(2) {
        println!("  i = {}", i);
    }

    // reverse iteration
    println!("Reverse iteration:");
    for i in (1..=5).rev() {
        println!("  i = {}", i);
    }

    // ignore value with _
    println!("Ignore value (print 3 times):");
    for _ in 0..3 {
        println!("  Hello!");
    }

    println!();
}

// 2. while loops - condition-based
fn while_loops() {
    println!("2. While Loops");

    // basic while loop
    println!("Basic while:");
    let mut count = 0;
    while count < 5 {
        println!("  count = {}", count);
        count += 1;
    }

    // while with condition
    println!("While with complex condition:");
    let mut x = 10;
    let mut y = 1;
    while x > y {
        println!("  x = {}, y = {}", x, y);
        x -= 1;
        y += 1;
    }

    // while let (pattern matching)
    println!("While let with Option:");
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(value) = stack.pop() {
        println!("  popped: {}", value);
    }

    // infinite while (careful!)
    println!("While true (with break):");
    let mut counter = 0;
    while true {
        counter += 1;
        println!("  counter = {}", counter);
        if counter >= 3 {
            break;
        }
    }

    println!();
}

// 3. loop - infinite loops (most flexible)
fn loop_loops() {
    println!("3. Loop (infinite loops)");

    // basic loop with break
    println!("Basic loop with break:");
    let mut i = 0;
    loop {
        println!("  i = {}", i);
        i += 1;
        if i >= 5 {
            break;
        }
    }

    // loop with return value
    println!("Loop with return value:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // return value
        }
    };
    println!("  result = {}", result);

    // loop with continue
    println!("Loop with continue (skip even numbers):");
    let mut num = 0;
    loop {
        num += 1;
        if num > 10 {
            break;
        }
        if num % 2 == 0 {
            continue; // skip rest of iteration
        }
        println!("  odd number: {}", num);
    }

    println!();
}

// 4. nested loops
fn nested_loops() {
    println!("4. Nested Loops");

    // nested for loops
    println!("Nested for loops (multiplication table):");
    for i in 1..=3 {
        for j in 1..=3 {
            print!("{:3}", i * j);
        }
        println!();
    }

    // nested loops with labels
    println!("Nested loops with labels:");
    'outer: for i in 1..=3 {
        'inner: for j in 1..=3 {
            if i == 2 && j == 2 {
                println!("  Breaking outer loop at ({}, {})", i, j);
                break 'outer; // break outer loop
            }
            println!("  ({}, {})", i, j);
        }
    }

    println!();
}

// 5. loop control (break, continue, labels)
fn loop_control() {
    println!("5. Loop Control");

    // break with value
    println!("Break with value:");
    let result = loop {
        let x = 42;
        break x + 10;
    };
    println!("  result = {}", result);

    // continue example
    println!("Continue (skip multiples of 3):");
    for i in 1..=10 {
        if i % 3 == 0 {
            continue;
        }
        println!("  {}", i);
    }

    // labeled breaks in nested loops
    println!("Labeled breaks:");
    'outer_loop: for x in 1..=3 {
        'inner_loop: for y in 1..=3 {
            if x == 2 && y == 2 {
                println!("  Breaking inner at ({}, {})", x, y);
                break 'inner_loop;
            }
            if x == 3 && y == 1 {
                println!("  Breaking outer at ({}, {})", x, y);
                break 'outer_loop;
            }
            println!("  Processing ({}, {})", x, y);
        }
    }

    println!("\n=== Key Concepts ===");
    println!("• for - iterate over ranges, arrays, vectors");
    println!("• while - condition-based loops");
    println!("• loop - infinite loops (most flexible)");
    println!("• break - exit loop (can return value)");
    println!("• continue - skip to next iteration");
    println!("• labels - control nested loops");
    println!("• .. vs ..= - exclusive vs inclusive ranges");
}
