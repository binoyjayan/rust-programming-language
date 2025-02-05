use futures::future::Future;
use futures::task::{Context, Poll};

use std::pin::Pin;

pub struct SimpleFuture {
    n: i32,
}

impl Future for SimpleFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.n)
    }
}

pub async fn simple_function(p: i32) -> i32 {
    p + 42
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{channel, executor, FutureExt};

    #[test]
    fn test_future_returns_a_value() {
        let future = SimpleFuture { n: 42 };
        // Since we do not have a context to call poll on the future,
        // we use the default executor to block on the future.
        let result = executor::block_on(future);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_future_returns_a_value_function() {
        let future = simple_function(42);
        let result = executor::block_on(future);
        assert_eq!(result, 84);
    }

    // Future combinators help in chaining multiple futures together
    #[test]
    fn test_future_combinators_chaining_1() {
        let future1 = SimpleFuture { n: 42 };
        let future2 = future1.map(|n| n * 2);
        let result = executor::block_on(future2);
        assert_eq!(result, 84);
    }

    #[test]
    fn test_future_combinators_chaining_2() {
        let future = SimpleFuture { n: 42 };
        let future = future
            .map(|n| n * 2)
            .map(|n| n + 10)
            .map(|n| n / 2)
            .map(|n| n - 5);
        let result = executor::block_on(future);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_future_combinators_chaining_function() {
        let future1 = simple_function(0);
        let future2 = future1.map(|n| n * 2);
        let result = executor::block_on(future2);
        assert_eq!(result, 84);
    }

    #[test]
    fn test_future_with_oneshot_channel() {
        let future = SimpleFuture { n: 42 };
        let (tx, rx) = channel::oneshot::channel();
        executor::block_on(future.map(move |n| tx.send(n))).unwrap();
        let result = executor::block_on(rx).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_async_send() {
        let (tx, rx) = channel::oneshot::channel();
        executor::block_on(async {
            let n = simple_function(42).await;
            tx.send(n)
        })
        .unwrap();
        let result = executor::block_on(async move {
            let n = rx.await.unwrap();
            n + 42
        });
        assert_eq!(result, 126);
    }
}
