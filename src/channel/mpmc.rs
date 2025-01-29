use std::sync::mpmc;

pub fn sync_channel<T>(bound: usize) -> (SenderWrapper<T>, ReceiverWrapper<T>) {
    let (s,r) = mpmc::sync_channel(bound);
    let s = SenderWrapper(s);
    let r = ReceiverWrapper(r);
    (s,r)
}

pub fn channel<T>() -> (SenderWrapper<T>, ReceiverWrapper<T>) {
    let (s,r) = mpmc::channel();
    let s = SenderWrapper(s);
    let r = ReceiverWrapper(r);
    (s,r)
}

pub struct SenderWrapper<T>(mpmc::Sender<T>);

impl<T> SenderWrapper<T> {
    pub fn send(&self, t: T) -> std::result::Result<(),mpmc::SendError<T>> {
        self.0.send(t)
    }
}

pub struct ReceiverWrapper<T>(pub mpmc::Receiver<T>);

impl<T> Clone for ReceiverWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> ReceiverWrapper<T> {
    pub fn recv(&self) -> Result<T, mpmc::RecvError> {
        self.0.recv()
    }
}
