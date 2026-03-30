fn main() {
    println!("Hello, world!");

    use std::{thread, time};
    let duration = time::Duration::from_millis(100);
    for i in 0..10 {
        println!("{}", i);
        thread::sleep(duration);
    }
}
