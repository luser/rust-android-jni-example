pub use anyhow::Result;
use futures::executor::ThreadPool;
pub use futures::executor::ThreadPoolBuilder;
use futures::prelude::*;
use futures::task::SpawnExt;

pub struct Driver {
    pool: ThreadPool,
}

impl Driver {
    pub fn new(pool: ThreadPool) -> Self {
        Driver { pool }
    }

    pub fn do_some_work<F: Future<Output = ()> + Send + 'static>(&self, work: F) -> Result<()> {
        self.pool.spawn(work)?;
        Ok(())
    }
}
