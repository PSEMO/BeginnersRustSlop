pub mod greetings;

use crate::greetings::formal::greet as formal_greet;
pub use crate::greetings::informal::greet as informal_greet;

pub fn formal_greet_caller() {
    formal_greet();
}