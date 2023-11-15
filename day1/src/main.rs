use std::fs;

const FILE_NAME: &'static str = "data.txt";

fn main() {
    let file = fs::read_to_string(FILE_NAME)
        .expect("should have been able to read the file");
        
    let lines = file.split('\n');

    let mut global_largest: u32 = 0;
    let mut local_counter: u32 = 0;

    for l in lines {
        if l.is_empty() {
            if local_counter > global_largest {
                println!("fount new global largest: {}", local_counter);
                global_largest = local_counter;
            }

            local_counter = 0;
        } else {
            local_counter += l.parse::<u32>().unwrap();
        }
    }

    println!("The largest is {}", global_largest);
}
