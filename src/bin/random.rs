use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let random_number = rng.random_range(0..100);
    println!("{}", random_number)
}
