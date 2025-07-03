use core::marker::PhantomData;
use core::mem;

use crate::worker::Job;
use crate::{Counter, ThreadPool};

/// A scope to spawn jobs inside a [ThreadPool]
///
/// This struct is created by the [ThreadPool::scope] function
pub struct Scope<'scope, 'pool: 'scope> {
    scope_counter: Counter,
    pool: &'pool ThreadPool,

    /// Invariance over 'scope, to make sure 'scope cannot shrink,
    /// which is necessary for soundness.
    ///
    /// Without invariance, this would compile fine but be unsound:
    ///
    /// ```compile_fail,E0373
    /// use job_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::default();
    /// pool.scope(|s| {
    ///     s.spawn(|| {
    ///         let a = String::from("abcd");
    ///         s.spawn(|| println!("{a:?}")); // might run after `a` is dropped
    ///     });
    /// });
    /// ```
    _marker_scope: PhantomData<&'scope mut &'scope ()>,
}

impl<'scope, 'pool> Scope<'scope, 'pool> {
    pub(super) fn new(pool: &'pool ThreadPool) -> Self {
        Self {
            scope_counter: Counter::new(),
            pool,
            _marker_scope: PhantomData,
        }
    }

    /// Executes a job inside this [Scope].
    pub fn execute(&self, job: impl Job<'scope>) {
        let job: Box<dyn Job<'scope>> = Box::new(job);
        /* SAFETY: Scope makes sure that all jobs sent through it are
         * finished before droping it. So the jobs won't outlive the
         * 'scope lifetime. */
        let job: Box<dyn Job<'static>> = unsafe { mem::transmute(job) };
        self.pool.execute_inside_scope(job, self.scope_counter.clone());
    }

    /// Creates a new scope inside `self`.
    ///
    /// # Example
    /// ```
    /// use job_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::default();
    ///
    /// let msg = String::from("Helloo :)");
    /// pool.scope(|scope| {
    ///     let helloworld = format!("{msg} world!");
    ///     scope.subscope(|subscope1| {
    ///         subscope1.execute(|| println!("1) {helloworld}"));
    ///         subscope1.execute(|| println!("2) {helloworld}"));
    ///     });
    /// });
    /// ```
    pub fn subscope<'new, F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Scope<'new, 'pool>) -> R,
        'scope: 'new
    {
        let scope = Scope {
            scope_counter: Counter::new(),
            pool: self.pool,
            _marker_scope: PhantomData,
        };
        f(&scope)
    }
}

impl Drop for Scope<'_, '_> {
    fn drop(&mut self) {
        self.scope_counter.join();
    }
}
