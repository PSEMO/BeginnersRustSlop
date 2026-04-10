enum GateStatus {
    Open,
    Closed,
    TimedOpen(u32),
    Maintenance(String)
}

fn check_access(input: Option<GateStatus>) {
    if let Some(status) = input {
        println!("===Something is found!===");
        match status {
            GateStatus::Open => println!("Open!"),
            GateStatus::Closed => println!("Closed!"),
            GateStatus::TimedOpen(time) => println!("will close in {}", time),
            GateStatus::Maintenance(warning) => println!("Maintenance! {}", warning),
        }
    }
    else {
        println!("=====Nothing Here=====");
        println!("Null");
    };
}

fn main() {
    check_access(Some(GateStatus::Open));
    check_access(Some(GateStatus::Closed));
    check_access(Some(GateStatus::TimedOpen(123u32)));
    check_access(Some(GateStatus::Maintenance(String::from("AFK brb"))));
    check_access(None);
}