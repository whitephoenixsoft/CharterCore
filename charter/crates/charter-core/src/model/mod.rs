pub mod area;
pub mod ids;
pub mod candidate;
pub mod resolution;
pub mod session;
pub mod vote;

enum CharterModelKind {
    Area(area::Area),
}
