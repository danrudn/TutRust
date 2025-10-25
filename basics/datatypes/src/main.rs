fn main() {

    // mutable and immutable
    {
        let mut mut_integer: i32 = 12;
        mut_integer = 50; // mutable -> OK
        let imut_integer: i32 = 12; 
        //immutInt = 50; // immutable -> not ok
    }

    // primitive typs
    {
        let integer: i32 = 42;
        let float: f64 = 3.14;
        let boolean: bool = true;
        println!("primitives: {integer} {float} {boolean}"); // println!, ! is for macro calls
    }
    // tuple array
    {
        let tuple: (i32, f64) = (10, 2.5);
        let array: [i32; 3] = [1, 2, 3]; // array [type, length]
        println!("Tuple: ({}, {})", tuple.0, tuple.1);
        println!("Array: {:?}", array);
    }

    // string
    {
        let str_slice: &str = "Hallo";
        let mut string: String = String::from("Rust");
        string.push('!');
        println!("Strings: {str_slice} {string}");
    }
    
    // shdowing
    {
        let x = 5;          
        let x = x * 2;      // new variable shadows  variable x from before
        println!("Variablen: x={x}");
    }

    // casting
    let num = 65;
    let num_f64: f64 = num as f64;
    println!("Casting: num:{num} -> num as f64: {num_f64}");
}
