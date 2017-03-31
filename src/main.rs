fn main() {
    let res = ::std::panic::catch_unwind(|| {
        panic!("Panic One!")
    });

    println!("Caught One: {:?}", res);

    let res = ::std::panic::catch_unwind(|| {
        panic!("Panic Two!")
    });

    println!("Caught Two: {:?}", res);
}
