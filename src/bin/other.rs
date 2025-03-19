fn main() {
    let mut book = String::from("Rust programming");
    let borrow = &mut book;

    *borrow = String::from("I have come again");
    println!("{}", book);
}
