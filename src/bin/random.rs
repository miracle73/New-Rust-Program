use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let a = rng.random_range(0..100);

    println!("{}", a);
}
