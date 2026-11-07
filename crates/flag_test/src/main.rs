use meine_enum::Flag;

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
    for i in Test::all() {
        println!("{i:?}");
    }
    println!("{}", size_of::<Test>());
    let s = ron::to_string(&(Test::A | Test::B)).unwrap();
    println!("{s} = {:?}", ron::from_str::<Test>(&s).unwrap());
    for i in Test::A | Test::C {
        println!("{i:?}");
    }
}
