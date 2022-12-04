use rand::prelude::*;
use std::{thread, time::Duration};

use wag::WaitGroup;
fn main() {
    let wg = WaitGroup::new();
    wg.adds::<10>().enumerate().for_each(|(i, child)| {
        thread::spawn(move || {
            let duration = rand::thread_rng().gen_range(0..500);
            thread::sleep(Duration::from_millis(duration));
            println!("sleep:{duration}, {i}");
            child.done();
        });
    });
    wg.wait();
}
