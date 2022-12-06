use std::fs;
const CALORIES_PATH: &str = "src/resources/calories.txt";

pub fn solve(){
    // read
    let contents = fs::read_to_string(CALORIES_PATH)
        .expect("Should have been able to read the file");
    // split when there's an empty row
    let str_list: Vec<&str> = contents.split("\n\n").collect();
    // convert to vector of integers (previously where "1\n2\n3")
    let int_list: Vec<Vec<u32>>  = str_list.iter().map(|x| 
        x.split("\n").map(|a| a.to_string().parse::<u32>().unwrap_or(0)).collect()
    ).collect();
    // sum calories each elf has
    let mut sum_list: Vec<u32> = int_list.iter().map(|ints| ints.iter().sum()).collect();
    // sort in dsecending order
    sum_list.sort_by(|a,b| b.cmp(a));
    // get top 3 elvers
    let top3 = &sum_list[0..3];
    // add them
    println!("first: {:?}", top3.iter().sum::<u32>());
}