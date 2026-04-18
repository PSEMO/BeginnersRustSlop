pub mod formal;

pub(super) fn internal_msg() {
    println!("System: Generating a greeting...");
}

pub mod informal {
    pub fn greet() {
        super::internal_msg();
        println!("WAZZAP BEIJING!");
    }
}

pub fn greet() {
    internal_msg();
    println!("UwU");
}