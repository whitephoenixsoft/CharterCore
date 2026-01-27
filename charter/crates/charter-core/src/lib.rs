pub mod engine;
pub mod storage;
pub mod model;
pub mod errors;
pub mod spec;
pub mod time;
pub mod types;

pub fn engine_version() -> &'static str {
	"charter-core 0.1.0"
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn sample_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .try_init();

        //test code here
    }
}
