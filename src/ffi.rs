use std::ptr;

use crate::ThreadPool;

#[repr(C)]
pub struct PoolConfig {
    n_workers: u16,
    max_jobs: i32,
    incoming_buf_size: i32,
}

impl PoolConfig {
    fn convert(&self) -> crate::PoolConfig {
        crate::PoolConfig {
            n_workers: self.n_workers,
            max_jobs: if self.max_jobs > 0 { Some(self.max_jobs as u16) } else { None },
            incoming_buf_size: if self.incoming_buf_size > 0 { Some(self.incoming_buf_size as u16) } else { None },
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C"
fn pool_default_conf() -> PoolConfig {
    PoolConfig {
        n_workers: 16,
        max_jobs: -1,
        incoming_buf_size: -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C"
fn pool_init(conf: PoolConfig) -> *mut ThreadPool {
    let conf = conf.convert();

    let Ok(pool) = ThreadPool::new(conf) else {
        return ptr::null_mut()
    };

    let pool = Box::new(pool);
    Box::into_raw(pool)
}

#[unsafe(no_mangle)]
pub extern "C"
fn pool_execute_job(pool: *mut ThreadPool, f: extern "C" fn ()) {
    unsafe {
        (*pool).execute(move || { f(); });
    }
}

#[unsafe(no_mangle)]
pub extern "C"
fn pool_join(pool: *mut ThreadPool) {
    unsafe { (*pool).join(); }
}

#[unsafe(no_mangle)]
pub extern "C"
fn pool_free(pool: *mut ThreadPool) {
    if pool.is_null() { return }
    unsafe {
        let pool = Box::from_raw(pool);
        drop(pool);
    }
}
