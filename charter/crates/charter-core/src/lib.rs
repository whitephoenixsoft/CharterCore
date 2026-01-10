pub mod engine;
pub mod storage;
pub mod model;
pub mod errors;
pub mod time;
pub mod types;

pub fn engine_version() -> &'static str {
	"charter-core 0.1.0"
}
