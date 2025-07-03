use crate::Result;

/// Pool Config
///
/// Configuration for the [ThreadPool](crate::ThreadPool)
#[derive(Clone,Copy,Debug)]
pub struct PoolConfig {
    pub n_workers: u16,
    pub max_jobs: Option<u16>,
    pub incoming_buf_size: Option<u16>,
}

impl PoolConfig {
    pub const fn builder() -> PoolConfigBuilder {
        PoolConfigBuilder {
            n_workers: 16,
            max_jobs: None,
            incoming_buf_size: None,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.n_workers == 0 {
            return Err("Invalid pool size: 0".into());
        }
        if let Some(max) = self.max_jobs {
            if max < self.n_workers {
                return Err(format!("Max number of jobs ({max}) is lower \
                        than the number of workers ({})", self.n_workers).into())
            }
        }
        Ok(())
    }
}

impl Default for PoolConfig {
    /// Default configuration
    ///
    /// Nº Workers: 16
    /// Max Jobs: None
    /// Incoming buf size: None
    fn default() -> Self {
        PoolConfig::builder().build()
    }
}

pub struct PoolConfigBuilder {
    n_workers: u16,
    max_jobs: Option<u16>,
    incoming_buf_size: Option<u16>,
}

impl PoolConfigBuilder {
    pub const fn n_workers(mut self, n: u16) -> Self {
        self.n_workers = n;
        self
    }
    pub const fn set_n_workers(&mut self, n: u16) -> &mut Self {
        self.n_workers = n;
        self
    }
    pub const fn max_jobs(mut self, n: u16) -> Self {
        self.max_jobs = Some(n);
        self
    }
    pub const fn set_max_jobs(&mut self, n: u16) -> &mut Self {
        self.max_jobs = Some(n);
        self
    }
    pub const fn incoming_buf_size(mut self, n: u16) -> Self {
        self.incoming_buf_size = Some(n);
        self
    }
    pub const fn set_incoming_buf_size(&mut self, n: u16) -> &mut Self {
        self.incoming_buf_size = Some(n);
        self
    }
    pub const fn build(self) -> PoolConfig {
        PoolConfig {
            n_workers: self.n_workers,
            incoming_buf_size: self.incoming_buf_size,
            max_jobs: self.max_jobs,
        }
    }
}
