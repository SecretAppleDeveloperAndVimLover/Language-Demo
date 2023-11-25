fn main() {
    println!("Hello, world!");
    let hello = 89;
    let world = 234;
    let hello_world = hello * world;
    println!("{}", hello_world);

    let mut pi = 0.0;
    let mut positive = true;
    for i in 0..1_000_000 {
        let term = 1.0 / (i as f64 * 2.0 + 1.0);
        if positive {
            pi += term;
        } else {
            pi -= term;
        }
        positive = !positive;
    }
    pi *= 4.0;
    println!("Approximation of Pi: {}", pi);
}
