use std::fs;

const FILE_NAME: &'static str = "data.txt";

fn main() {
    let file = fs::read_to_string(FILE_NAME)
        .expect("should have been able to read the file");
        
    let lines = file.split('\n');

    let mut vec = vec![];
    let mut counter: u32 = 0;

    for l in lines {
        if l.is_empty() {
            vec.push(counter);
            counter = 0;
        } else {
            counter += l.parse::<u32>().unwrap();
        }
    }

    vec.sort();
    vec.reverse();

    assert!(vec.len() >= 3);

    let total = vec[0] + vec[1] + vec[2];

    println!("The top three values summed are: {}", total);
}
