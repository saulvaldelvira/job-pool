use crate::scope::Scope;
use crate::worker::{Job, Worker, Message};
use crate::{channel, Counter, PoolConfig, Result};
use crate::channel::SenderWrapper;

/// Thread Pool
///
/// A thread pool coordinates a group of threads to run
/// taks in parallel.
///
/// # Example
/// ```
/// use job_pool::ThreadPool;
///
/// let pool = ThreadPool::with_size(32).expect("Error creating pool");
/// pool.execute(|| println!("Hello world!"));
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: SenderWrapper<Message>,
    job_count: Counter,
    max_jobs: Option<u16>,
}

impl ThreadPool {
    /// Creates a new `ThreadPool`
    ///
    /// # Errors
    /// If the [PoolConfig] is not valid
    pub fn new(config: PoolConfig) -> Result<ThreadPool> {
        config.validate()?;

        let size = config.n_workers as usize;
        let (sender,receiver) =
            if let Some(max) = config.incoming_buf_size {
                channel::sync_channel(max as usize)
            } else {
                channel::channel()
            };
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size-1 {
            let worker = Worker::new(receiver.clone());
            workers.push(worker);
        }
        let worker = Worker::new(receiver);
        workers.push(worker);

        let global = Counter::new();
        Ok(ThreadPool {
            workers,
            job_count: global,
            max_jobs: config.max_jobs,
            sender,
        })
    }
    /// Create a [ThreadPool] with the default [configuration](PoolConfig)
    #[inline]
    pub fn with_default_config() -> Self {
        let conf = PoolConfig::default();
        Self::new(conf).expect("The default config is valid")
    }
    /// Create a [ThreadPool] with a given size
    #[inline]
    pub fn with_size(size: u16) -> Result<Self> {
        let conf = PoolConfig::builder()
                              .n_workers(size)
                              .build();
        Self::new(conf)
    }

    /// Returns the number of pending jobs
    pub fn pending_jobs(&self) -> usize {
        self.job_count.count() as usize
    }

    pub(crate) fn execute_inside_scope(&self, job: Box<dyn Job<'static>>, scope_counter: Counter) {
        self.job_count.inc(self.max_jobs);
        scope_counter.inc(None);

        let msg = Message::Job {
            job: Box::new(job),
            global_counter: self.job_count.clone(),
            scope_counter: Some(scope_counter),
        };
        self.sender.send(msg).unwrap()
    }

    /// Executes the given job inside this pool.
    ///
    /// # Example
    /// ```
    /// use job_pool::ThreadPool;
    ///
    /// fn heavy_computation(n: u64) -> u64 {
    ///     // ....
    ///     n
    /// }
    ///
    /// let pool = ThreadPool::default();
    /// pool.execute(|| {
    ///     println!("JOB1: {}", heavy_computation(1));
    /// });
    ///
    /// pool.execute(|| {
    ///     println!("JOB2: {}", heavy_computation(2));
    /// });
    /// ```
    pub fn execute(&self, job: impl Job<'static>) {
        self.job_count.inc(self.max_jobs);
        let msg = Message::Job {
            job: Box::new(job),
            global_counter: self.job_count.clone(),
            scope_counter: None
        };
        self.sender.send(msg).unwrap();
    }

    /// Creates a new [Scope] to spawn jobs.
    ///
    /// All the jobs spawned via [Scope::execute], will be joined
    /// when the scope drops.
    ///
    /// # Example
    /// ```
    /// use job_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::default();
    ///
    /// let msg = String::from("Helloo :)");
    /// pool.scope(|scope| {
    ///     scope.execute(|| {
    ///         println!("I'm job1, borrowing {msg:?}");
    ///     });
    ///     scope.execute(|| {
    ///         println!("I'm job2, borrowing {msg:?}");
    ///     });
    /// });
    ///
    /// // At this point, all the jobs spawned inside the scope above
    /// // are done. That's wy it is ok to borrow msg, because we make
    /// // sure that the jobs don't outlive the scope's lifetime.
    /// ```
    pub fn scope<'scope, 'pool, F, R>(&'pool self, f: F) -> R
    where
        F: FnOnce(&Scope<'scope, 'pool>) -> R,
        'pool: 'scope
    {
        let scope = Scope::new(self);
        f(&scope)
    }

    /// Waits for all the jobs in the pool to finish
    pub fn join(&self) {
        self.job_count.join();
    }
}

impl Drop for ThreadPool  {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Shutdown).unwrap();
        }

        for worker in &mut self.workers {
            worker.shutdown();
        }
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self::with_default_config()
    }
}
