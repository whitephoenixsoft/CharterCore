use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static OBJECT_COUNTER: AtomicU64 = AtomicU64::new(1);

pub type ExternalId = String;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(pub [u8 ; 32]);

impl fmt::Debug for ObjectId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Object({})", self.to_hex())
	}
}

impl fmt::Display for ObjectId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

impl ObjectId {
	/// Generates a determistic, process-local ObjectId.
	/// This will be replaced by content-addressed IDs later.
	pub fn new() -> Self {
		let mut bytes = [0u8 ; 32];
		let n = OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst);
		bytes[..8].copy_from_slice(&n.to_be_bytes());
		ObjectId(bytes)
	}

	/// Returns a lowercase hex representatin.
	pub fn to_hex(&self) -> String {
		self.0.iter().map(|b| format!("{:02x}", b)).collect()
	}
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AreaId(pub ObjectId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SessionId(pub ObjectId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ResoutionId(pub ObjectId);
