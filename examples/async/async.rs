use rand::prelude::*;

use wag::WaitGroup;
#[tokio::main]
async fn main() {
    let wg = WaitGroup::new();

    wg.adds_iter::<10>().for_each(|child| {
        tokio::spawn(async move {
            let duration = rand::thread_rng().gen_range(0..500);
            tokio::time::sleep(std::time::Duration::from_millis(duration)).await;
            println!("async sleep:{duration}");
            child.done();
        });
    });
    wg.async_wait().await;
}
