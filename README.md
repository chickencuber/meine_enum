# Meine Enum
>> an Enum-based Helper crate

## Install
```toml
meine_enum = "*" # or whatever the current version is
# or 
meine_enum = { 
    version = "1.0", # or whatever the current version is
    features = ["standard"]
}
```

## Features
### Flag enum
- transform an enum into a bitflag type with an ergonomic api
- it has derive pass through and an enum member is iterable
- it automatically picks the smallest possible uint type for the given enum 
- now supports enums with more than 128 variants, I am confident enough to say that it will work for large variant lengths, if you have 50,000 variants and it doesn't work… why are you using 50,000 variants
```rust
use meine_enum::Flag; //or meine_enum::flag if using feature=["standard"]

#[Flag]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Test {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
}

fn main() {
    for i in Test::all() { // a member with every flag, members are iterable
        println!("{i:?}");
    }
    println!("{}", size_of::<Test>());
    let s = ron::to_string(&(Test::A | Test::B)).unwrap();
    println!("{s} = {:?}", ron::from_str::<Test>(&s).unwrap());
    for i in Test::A | Test::C { // you can also do this
        println!("{i:?}");
    }
}
```
#### Supported Operators
- "!"
- "^"
- "&"
- "|"
- "-"
- "-="
- "^="
- "&="
- "|="


## Planned 
- add more features besides just the flag enum

[TODO](./TODO.md)
