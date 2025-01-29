use std::sync::mpsc::SendError;
use std::sync::{mpsc, Arc, Mutex};

pub fn sync_channel<T>(bound: usize) -> (SenderWrapper<T>, ReceiverWrapper<T>) {
    let (s,r) = mpsc::sync_channel(bound);
    let s = SenderWrapper::Bounded(s);
    let r = ReceiverWrapper(Arc::new(Mutex::new(r)));
    (s,r)
}

pub fn channel<T>() -> (SenderWrapper<T>, ReceiverWrapper<T>) {
    let (s,r) = mpsc::channel();
    let s = SenderWrapper::Unbounded(s);
    let r = ReceiverWrapper(Arc::new(Mutex::new(r)));
    (s,r)
}

pub enum SenderWrapper<T> {
    Bounded(mpsc::SyncSender<T>),
    Unbounded(mpsc::Sender<T>),
}

impl<T> SenderWrapper<T> {
    pub fn send(&self, t: T) -> std::result::Result<(),SendError<T>> {
        match self {
            SenderWrapper::Bounded(b) => b.send(t),
            SenderWrapper::Unbounded(u) => u.send(t),
        }
    }
}

pub struct ReceiverWrapper<T>(pub Arc<Mutex<mpsc::Receiver<T>>>);

impl<T> Clone for ReceiverWrapper<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> ReceiverWrapper<T> {
    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        self.0.lock().unwrap().recv()
    }
}
