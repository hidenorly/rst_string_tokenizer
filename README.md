# rst_string_tokenizer

StringTokenizer implementation in Rust as this is well-known in Java.

```
    let target_string = "Hello,_,world,_,from,_,rust!,_,";

    let mut token = StringTokenizer::new( &target_string, ",_," );

    while token.has_next() {
        println!( "{}", token.get_next() );
    }
```

# TODO

* [Done] Make this as library
* [Done] Add test cases
* [Done] Add Trim

