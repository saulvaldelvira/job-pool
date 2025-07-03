/*  Copyright (C) 2025 Saúl Valdelvira
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, version 3.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>. */

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

#![cfg_attr(feature = "use-nightly-mpmc", feature(mpmc_channel))]

#[cfg(feature = "bindings")]
mod ffi;

mod pool;
mod worker;
mod config;
mod scope;
pub use scope::Scope;

/* Switch between mpsc and mpmc until
 * std::sync::mpmc is stabilized */

#[cfg(feature = "use-nightly-mpmc")]
#[path ="channel/mpmc.rs"]
mod channel;

#[cfg(not(feature = "use-nightly-mpmc"))]
#[path ="channel/mpsc.rs"]
mod channel;

use std::borrow::Cow;
use std::sync::{Arc, Condvar, Mutex};

pub use pool::ThreadPool;
pub use config::PoolConfig;

pub type Result<T> = std::result::Result<T,Cow<'static,str>>;

#[derive(Clone)]
struct Counter(Arc<(Mutex<u16>,Condvar)>);

impl Counter {
    pub fn new() -> Self {
        Self(Arc::new((Mutex::new(0), Condvar::new())))
    }

    pub fn inc(&self, max: Option<u16>) {
        let (lock,cvar) = &*self.0;
        let mut counter = lock.lock().unwrap();
        if let Some(max) = max {
            counter = cvar.wait_while(counter, |n| *n >= max).unwrap();
        }
        *counter += 1;

    }

    pub fn count(&self) -> u16 {
        *self.0.0.lock().unwrap()
    }

    pub fn dec(&self) {
        let (lock,condv) = &*self.0;
        let mut counter = lock.lock().unwrap();
        *counter -= 1;
        condv.notify_one();
    }

    pub fn join(&self) {
        let (lock,cvar) = &*self.0;
        let counter = lock.lock().unwrap();
        let _lock = cvar.wait_while(counter, |n| *n > 0).unwrap();
    }
}
