
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
