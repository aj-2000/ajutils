use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use std::fs;
use std::io;


fn main() {
    std::process::exit(real_main());
}
fn convert_bytes_to_MB(input: u64) -> u64 {
    return input / (1024*1024);
}
fn real_main() -> i32 {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <option>", args[0]);
        return 1;
    }
    if args[1]=="free_ram"{
        let mut sys = System::new_all();
        sys.refresh_all();
        println!("Free RAM : {} MB / {} MB", convert_bytes_to_MB(sys.free_memory()), convert_bytes_to_MB(sys.total_memory()));

    }
    if args[1]=="version"{
        let mut sys = System::new_all();
        sys.refresh_all();
        println!("AJ Utils 0.01a");

    }
    0
}