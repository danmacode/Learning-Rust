
### Split a str in a vec
```
let vec = split.collect::<Vec<&str>>(); // specify what type we collect
let vec: Vec<&str> = split.collect(); // specify type of vector
let vec: Vec<&str> = split.collect::<Vec<&str>>(); // specify everything (optional)
```
### Str 2 u32
```
let four: Result<u32,ParseIntError> = "5".parse::<u32>();
    let four2 :u32 = "a".parse().unwrap(); // faster, but can fail: 
    //                                        thread'main' panicked [ParseIntError]
```
