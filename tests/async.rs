use rand::prelude::*;
use std::time::Duration;
use wag::WaitGroup;

async fn async_sleep() -> Duration {
    let duration = Duration::from_millis(rand::thread_rng().gen_range(0..500));
    tokio::time::sleep(duration).await;
    duration
}

#[tokio::test]
async fn it_works_for_async_simple_add() {
    let wg = WaitGroup::new();
    for i in 0..10 {
        let w = wg.add();

        tokio::spawn(async move {
            println!("sleep:{}, {i}", async_sleep().await.as_millis());
            w.done();
        });
    }
    wg.async_wait().await;
}

#[tokio::test]
async fn it_works_for_async_adds() {
    let wg = WaitGroup::new();
    let [w1, w2, w3] = wg.adds();

    tokio::spawn(async move {
        println!("sleep:{}, 1", async_sleep().await.as_millis());
        w1.done();
    });

    tokio::spawn(async move {
        println!("sleep:{}, 2", async_sleep().await.as_millis());
        w2.done();
    });

    tokio::spawn(async move {
        println!("sleep:{}, 3", async_sleep().await.as_millis());
        w3.done();
    });

    wg.async_wait().await;
}

#[tokio::test]
async fn it_works_for_async_adds_as_iter() {
    let wg = WaitGroup::new();

    for w in wg.adds::<10>() {
        tokio::spawn(async move {
            println!("sleep:{}, 1", async_sleep().await.as_millis());
            w.done();
        });
    }

    wg.async_wait().await;
}

#[tokio::test]
async fn it_works_for_async_adds_iter() {
    let wg = WaitGroup::new();
    wg.adds_iter::<10>().enumerate().for_each(|(i, w)| {
        tokio::spawn(async move {
            println!("sleep:{}, {i}", async_sleep().await.as_millis());
            w.done();
        });
    });
    wg.async_wait().await;
}
