pub mod area;
pub mod audit_store;
pub mod core;
pub mod envelope;
pub mod hashing;
pub mod memory;
pub mod metadata_store;
pub mod object_store;
pub mod ref_store;

pub use area::AreaRoot;

use crate::model::ids::*;
use serde::Serialize;
use strum::Display;

#[serde(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub struct ObjectHash(pub String);

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "lowercase")]
#[serde(tag = "object_type", content = "object", rename_all = "lowercase")]
pub enum CharterObjectKind {
    Area(AreaRoot),
    //Session,
    //Resolution,
}

// Map Domain -> Persistence
impl From<CharterModelKind> for CharterObjectKind {
    fn from(o: CharterModelKind) -> Self {
        match o {
            CharterModelKind::Area(a) => CharterObjectKind::Area(AreaRoot::from(a)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

    /*
    #[test]
    pub fn to_string_area() {
        assert_eq!("area", CharterObjectKind::Area.to_string())
    }
    */
}

/* Example pattern
 * 

// 1. Separate data records (can be saved directly)
struct Car { make: String, model: String }
struct Truck { payload_capacity: u32 }

// 2. Domain enum for application logic
enum Vehicle {
    Car(Car),
    Truck(Truck),
}

// 3. Mapping Logic
impl From<Car> for Vehicle {
    fn from(car: Car) -> Self {
        Vehicle::Car(car)
    }
}

// Example usage for saving:
fn save_vehicle(vehicle: Vehicle) {
    match vehicle {
        Vehicle::Car(car_data) => {
            // car_data is just the 'Car' struct, save it to object store
            save_to_store(car_data);
        }
        Vehicle::Truck(truck_data) => {
            save_to_store(truck_data);
        }
    }
}

//another example

// Domain Model
enum Vehicle {
    Car { make: String, doors: u8 },
}

// Persistence Model (optimized for saving)
struct CarRecord {
    id: uuid::Uuid,
    brand_name: String,
    door_count: i32,
}

// The "Bridge"
impl From<Vehicle> for CarRecord {
    fn from(v: Vehicle) -> Self {
        match v {
            Vehicle::Car { make, doors } => CarRecord {
                id: uuid::Uuid::new_v4(), // Persistence detail
                brand_name: make,
                door_count: doors as i32,
            }
        }
    }
}

 * 
 */
