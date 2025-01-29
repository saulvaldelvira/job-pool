//! Thread Pool
//!
//! This crate contains code to run a Job pool.
//!
//! # Example
//! ```rust,no_run
//! use job_pool::*;
//! use std::thread;
//! use std::time::Duration;
//!
//! let conf = PoolConfig::default();
//! let pool = ThreadPool::new(conf).unwrap();
//! for _ in 0..10 {
//!     pool.execute(|| {
//!         thread::sleep(Duration::from_secs(5));
//!     });
//! }
//! pool.join();
//! ```

#![cfg_attr(rustc_nightly, feature(mpmc_channel))]

#[cfg(feature = "bindings")]
mod ffi;

mod pool;
mod worker;
mod config;

/* Switch between mpsc and mpmc until
 * std::sync::mpmc is stabilized */

#[cfg(rustc_nightly)]
#[path ="channel/mpmc.rs"]
mod channel;

#[cfg(not(rustc_nightly))]
#[path ="channel/mpsc.rs"]
mod channel;

use std::{borrow::Cow, sync::{Arc, Condvar, Mutex}};

pub use pool::ThreadPool;
pub use config::PoolConfig;

type Semaphore = Arc<(Mutex<u16>,Condvar)>;

pub type Result<T> = std::result::Result<T,Cow<'static,str>>;
