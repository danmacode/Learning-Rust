
### Split a str in a vec
```
    let vec = split.collect::<Vec<&str>>(); // specify what type we collect
    let vec: Vec<&str> = split.collect(); // specify type of vector
    let vec: Vec<&str> = split.collect::<Vec<&str>>(); // specify everything (optional)
```
### Str tu u32 
https://doc.rust-lang.org/std/primitive.str.html#method.parse
```
    let four: Result<u32,ParseIntError> = "5".parse::<u32>();
    let four2 :u32 = "a".parse().unwrap(); // faster, but can fail: 
    //                                        thread'main' panicked [ParseIntError]
```
```
let number: i32 = match "15".parse::<i32>() { Ok(number)  => number,
        Err(e) => -1, };
```
### Dictionary
There are not native dictionaries but we can use HashMaps
```
use std::collections::HashMap;
let countries: HashMap<&str, i32> =
        [("Norway", 100),
         ("Denmark", 50),
         ("Iceland", 10)]
         .iter().cloned().collect();
vs
const COUNTRIES: [(&str, i32);6] = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)];
let countries: HashMap<&str, i32> = COUNTRIES.iter().cloned().collect();
vs                                                            _ _ can be substituted by &str, i32
let mut hs:HashMap<&str, i32> = std::collections::HashMap::<_, _>::new(); // leaves HashMap ??
    for &(key, value) in COUNTRIES.iter() {
        hs.insert(key, value);
    }
```
### Split string
```
let mut split = "10 0 0 2".split(" ");
//for s in split { println!("{}", s) } // print each String
let vec: Vec<&str> = split.collect::<Vec<&str>>(); // put them in a vector
```
### Switch
```
match total {
        // Match a single value
        1 => return "B".to_string(),
        // Match an inclusive range
        13..=19 => {
            println!("A teen.");
            println!("Ranges from 13 to 19.");
        }
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
```
