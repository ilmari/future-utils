use futures::{Async, Future, Poll};

/// Like repeatedly calling `.and_then(func)` as long as `pred` returns `true`
pub struct AndThenWhile<A, P, F> {
    inner: A,
    pred: P,
    func: F,
}

impl<A, P, F> AndThenWhile<A, P, F> {
    pub fn new(inner: A, pred: P, func: F) -> Self {
        AndThenWhile { inner, pred, func }
    }
}

impl<A, P, F> Future for AndThenWhile<A, P, F>
where
    A: Future,
    P: FnMut(&A::Item) -> bool,
    F: FnMut(A::Item) -> A,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<A::Item, A::Error> {
        let v = try_ready!(self.inner.poll());

        if (self.pred)(&v) {
            self.inner = (self.func)(v);
            Ok(Async::NotReady)
        } else {
            Ok(Async::Ready(v))
        }
    }
}
