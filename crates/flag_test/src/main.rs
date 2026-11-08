use meine_enum::{
    Flag, 
    test, // only exists when meine_enum has features=["test"]
};

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

test!(Test2, 129); //creates a flag enum 'Test2' with 129 varients named A[0..129](aka: 0..=128)

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

    println!("{:?}", Test2::A128|Test2::A125);
    println!("{}", size_of::<Test2>());
    println!("{}", Test2::all().count());
}
