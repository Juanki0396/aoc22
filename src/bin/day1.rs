use aoclib::elves::CaloriesReader;
use aoclib::inputs;

///Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn main() {
    let mut path = std::env::current_exe().expect("Cannot read the path");
    path.pop();
    path.push("../../inputs/day1_1.txt");
    println!("{:?}", path);
    let file_content = inputs::read_file(path.as_path());
    let calories_reader = CaloriesReader::build(&file_content);

    // Calories per elve
    let mut sums: Vec<i32> = calories_reader.map(|item| item.iter().sum()).collect();
    println!("The max calories is: {}", sums.iter().max().unwrap());

    sums.sort();
    let vec_len = sums.len();
    println!(
        "Total 3 max elves calories: {}",
        &sums[vec_len - 3..vec_len].iter().sum() as &i32
    );
}
