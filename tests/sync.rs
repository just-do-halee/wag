use rand::prelude::*;
use std::{thread, time::Duration};
use wag::WaitGroup;

fn sleep() -> Duration {
    let duration = Duration::from_millis(rand::thread_rng().gen_range(0..500));
    thread::sleep(duration);
    duration
}

#[test]
fn it_works_for_simple_add() {
    let wg = WaitGroup::new();
    for i in 0..10 {
        let w = wg.add();

        thread::spawn(move || {
            println!("sleep:{}, {i}", sleep().as_millis());
            w.done();
        });
    }
    wg.wait();
}

#[test]
fn it_works_for_adds() {
    let wg = WaitGroup::new();
    let [w1, w2, w3] = wg.adds();

    thread::spawn(move || {
        println!("sleep:{}, 1", sleep().as_millis());
        w1.done();
    });

    thread::spawn(move || {
        println!("sleep:{}, 2", sleep().as_millis());
        w2.done();
    });

    thread::spawn(move || {
        println!("sleep:{}, 3", sleep().as_millis());
        w3.done();
    });

    wg.wait();
}

#[test]
fn it_works_for_adds_as_iter() {
    let wg = WaitGroup::new();

    for w in wg.adds::<10>() {
        thread::spawn(move || {
            println!("sleep:{}, 1", sleep().as_millis());
            w.done();
        });
    }

    wg.wait();
}

#[test]
fn it_works_for_adds_iter() {
    let wg = WaitGroup::new();
    wg.adds_iter::<10>().enumerate().for_each(|(i, w)| {
        thread::spawn(move || {
            println!("sleep:{}, {i}", sleep().as_millis());
            w.done();
        });
    });
    wg.wait();
}
