pub use anyhow::Result;
use futures::executor::{ThreadPool, ThreadPoolBuilder};
use futures::prelude::*;
use futures::task::SpawnExt;

pub struct Driver {
    pool: ThreadPool,
}

impl Driver {
    pub fn new<F: Fn() + Send + Sync + 'static>(after_thread_start: F) -> Result<Self> {
        let mut builder = ThreadPoolBuilder::new();
        let pool = builder
            .after_start(move |_| after_thread_start())
            .create()?;
        Ok(Driver { pool })
    }

    pub fn do_some_work<F: Future<Output = ()> + Send + 'static>(&self, work: F) -> Result<()> {
        self.pool.spawn(work)?;
        Ok(())
    }
}
