use std::thread::{JoinHandle,spawn};
use crate::channel::ReceiverWrapper;
use crate::Counter;

/// A message sent to the [Worker]
pub enum Message {
    /// A Job to be executed
    Job {
        /// The job to be run
        job: Box<dyn Job<'static>>,
        /// The global [Counter] of jobs
        global_counter: Counter,
        /// The [Counter] of jobs for the [Scope](crate::scope::Scope)
        scope_counter: Option<Counter>,
    },
    Shutdown,
}

/// Type of function ran by the [Worker]
pub trait Job<'scope>: FnOnce() + Send + 'scope {}
impl<'scope, T> Job<'scope> for T
where T: FnOnce() + Send + 'scope {}

/// Worker for the [ThreadPool](crate::ThreadPool)
pub struct Worker(Option<JoinHandle<()>>);

impl Worker {
    /// Creates a new [Worker]
    pub fn new(
        receiver: ReceiverWrapper<Message>,
    ) -> Worker {
        let thread = spawn(move || loop {
            let message = receiver.recv();

            match message {
                Ok(Message::Job { job, global_counter, scope_counter }) => {
                    job();
                    global_counter.dec();
                    if let Some(scope) = scope_counter {
                        scope.dec();
                    }
                }
                Ok(Message::Shutdown) => break,
                Err(err) => panic!("Receive error: {err}"),
            }
        });
        Worker(Some(thread))
    }
    /// Shuts down the [Worker]
    pub fn shutdown(&mut self) {
        if let Some(thread) = self.0.take() {
            thread.join().unwrap();
        }
    }
}
