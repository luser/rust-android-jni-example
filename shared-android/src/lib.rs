use futures_timer::Delay;
use once_cell::sync::Lazy;
use shared_core::{Driver, Result};
use std::sync::{Arc, Mutex};
use std::time::Duration;

static DRIVER: Lazy<Arc<Mutex<Option<Driver>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

fn java_init_thread() -> Result<()> {
    //TODO: actually init JNI thread
    Ok(())
}

pub fn do_work_from_jni() -> Result<()> {
    //TODO: take a JNIEnv and Java object as parameters
    let driver = Driver::new(move || {
        java_init_thread().expect("Failed to init JNI thread");
    })?;
    driver.do_some_work(async {
        // Pretend we're waiting for some real async operation.
        Delay::new(Duration::from_secs(1)).await
        //TODO: call back into Java via JNI
    })?;
    DRIVER.lock().unwrap().replace(driver);
    Ok(())
}
