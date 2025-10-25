fn main() {
    // vector
    println!("vector");
    {
        let mut numbers: Vec<i32> = vec![1, 2, 3];
        numbers.push(4);
        numbers.push(5);

        println!("Vec: {:?}", numbers);
        println!("Länge: {}, Erstes: {:?}", numbers.len(), numbers.get(0));
    }

    // hashmap
    println!("hasmap");
    {
        use std::collections::HashMap;
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert("Alice".to_string(), 95); // only "Alice" would not work because it is string slice (static text) not a String-Type
        scores.insert("Bob".to_string(), 87);
        for (key, value) in &mut scores {
            println!("key:{key}, value{value}");
        }
    }

    // hashset
    println!("hashset");
    {
        use std::collections::HashSet;
        let mut unique_numbers: HashSet<i32> = HashSet::new();
        unique_numbers.insert(1);
        unique_numbers.insert(2);
        unique_numbers.insert(1); // Duplikat ignoriert
    }

    // deque
    println!("deque");
    {
        use std::collections::VecDeque;
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back("Hinten");
        queue.push_front("Vorne");
        queue.pop_front();
    }

    // map
    println!("map");
    {
        use std::collections::BTreeMap;
        let mut sorted_map: BTreeMap<i32, String> = BTreeMap::new();
        sorted_map.insert(3, "Drei".to_string());
        sorted_map.insert(1, "Eins".to_string());
        sorted_map.insert(2, "Zwei".to_string());

        for (key, value) in &mut sorted_map {
            // &sorted_map: reference to the map
            // &key is a reference to the key in the map,
            // &&value is a referenz to the value in the map which is a reference to the string slice -> reference to a reference
            *value = "Hallo".to_string(); // dereference with *
            println!("{key}: {value}");
        }
    }

    // loops over container
    println!("loops over container");
    {
        {
            // - simple iteration
            let vec: Vec<u32> = vec![1, 2, 3, 4, 5];
            for &value in &vec {
                print!("{} ", value);
            }
            println!();
        }

        {
            // - for each
            let vec: Vec<u32> = vec![1, 2, 3, 4, 5];
            vec.iter().for_each(|&x| print!("{x}")); // |&x| is a closure for lambda
            println!();
        }

        {
            // - iterator
            let mut vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
            let mut iterator = vec.iter_mut(); // mutable iterator, IterMut<'_, u32> this '_ is the lifetime of the iterator (avoid dangling pointers) 

            while let Some(value) = iterator.next() {
                // Some is a option in Rust. Option{Some<T>, None}. So only values a printed. When the ierators reaches the end it returns None -> not printed
                print!("{value}");
            }
            println!();
        }
    }
}
