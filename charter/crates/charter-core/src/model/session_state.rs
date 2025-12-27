#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Paused,
    Blocked,
    Closed,
}
