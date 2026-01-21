pub mod area;
pub mod ids;
pub mod candidate;
pub mod resolution;
pub mod session;
pub mod vote;

pub enum CharterModelKind {
    Area(area::Area),
}
