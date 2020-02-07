

```
let vec = split.collect::<Vec<&str>>(); // specify what type we collect
let vec: Vec<&str> = split.collect(); // specify type of vector
let vec: Vec<&str> = split.collect::<Vec<&str>>(); // specify everything
```
