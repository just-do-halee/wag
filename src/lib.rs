use std::sync::Arc;

pub struct WaitChild(Arc<()>);
impl WaitChild {
    pub fn done(self) {}
}

#[derive(Default)]
pub struct WaitGroup(Arc<()>);

impl WaitGroup {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&self) -> WaitChild {
        WaitChild(self.0.clone())
    }
    pub fn adds<const N: usize>(&self) -> [WaitChild; N] {
        [(); N].map(|_| self.add())
    }
    pub fn adds_iter<const N: usize>(&self) -> impl Iterator<Item = WaitChild> + '_ {
        (0..N).map(|_| self.add())
    }
    pub fn wait(self) {
        use std::{thread::sleep, time::Duration};

        while Arc::strong_count(&self.0) > 1 {
            sleep(Duration::from_millis(1));
        }
    }
    pub async fn async_wait(self) {
        WaitFuture(self.0).await
    }
}

// -------------- Async --------------
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct WaitFuture(Arc<()>);
impl Future for WaitFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Arc::strong_count(&self.0) > 1 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
