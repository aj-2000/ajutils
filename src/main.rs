use sysinfo::{System, SystemExt};
use rand::Rng;

fn main() {
    std::process::exit(real_main());
}
fn convert_bytes_to_mb(input: u64) -> u64 {
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
        println!("Free RAM : {} MB / {} MB", convert_bytes_to_mb(sys.free_memory()), convert_bytes_to_mb(sys.total_memory()));

    }

    if args[1]=="version" {
        let mut sys = System::new_all();
        sys.refresh_all();
        println!("AJ Utils 0.01a");
    }

    if args[1]=="rand_num" {
        let min: i64 = args[2].parse::<i64>().unwrap();
        let max: i64 = args[3].parse::<i64>().unwrap();
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(min..max);
        println!("Random number betweeen {} and {} : {} \n", min, max, rand_num);
    }
    0
}

//random number
//favs commands
