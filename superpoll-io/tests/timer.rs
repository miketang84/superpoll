use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use superpoll_io::Timer;
use futures::{future, executor, prelude::*};

fn spawn<T: Send + 'static>(
    f: impl Future<Output = T> + Send + 'static,
) -> impl Future<Output = T> + Send + 'static {
    let (mut s, mut r) = futures::channel::mpsc::channel(1);

    thread::spawn(move || {
        executor::block_on(async {
            s.send(f.await).await.ok();
        })
    });

    Box::pin(async move { r.next().await.unwrap() })
}

#[test]
fn smoke() {
    executor::block_on(async {
        let start = Instant::now();
        Timer::after(Duration::from_secs(1)).await;
        assert!(start.elapsed() >= Duration::from_secs(1));
    });
}

#[test]
fn poll_across_tasks() {
    executor::block_on(async {
        let start = Instant::now();
        let (mut sender, mut receiver) = futures::channel::mpsc::channel(1);

        let task1 = spawn(async move {
            let timer = Timer::after(Duration::from_secs(1));

//            async {
//                (&mut timer).await;
//                panic!("timer should not be ready")
//            }
//            .or(async {})
//            .await;

            sender.send(timer).await.ok();
        });

        let task2 = spawn(async move {
            let timer = receiver.next().await.unwrap();
            timer.await;
        });

        task1.await;
        task2.await;

        assert!(start.elapsed() >= Duration::from_secs(1));
    });
}

#[test]
fn set() {
    executor::block_on(async {
        let start = Instant::now();
        let timer = Arc::new(Mutex::new(Timer::after(Duration::from_secs(10))));

        thread::spawn({
            let timer = timer.clone();
            move || {
                thread::sleep(Duration::from_secs(1));
                timer.lock().unwrap().set_after(Duration::from_secs(2));
            }
        });

        future::poll_fn(|cx| Pin::new(&mut *timer.lock().unwrap()).poll(cx)).await;

        assert!(start.elapsed() >= Duration::from_secs(2));
        assert!(start.elapsed() < Duration::from_secs(10));
    });
}
