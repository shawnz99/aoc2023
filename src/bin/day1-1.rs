use std::fs::read_to_string;

fn main() {
    let file_path = "../../inputs/day1in.txt";
    println!("In file {:?}", file_path);



    let mut total_sum = 0;
    let mut ii = 1;
    for line in read_to_string(file_path).unwrap().lines() {
        let mut line_sum = String::from("");
        let mut first_num = 'z';
        let mut second_num = 'z';
        for (_i,char) in line.chars().enumerate() {
            if char.is_digit(10) && first_num == 'z' {
                first_num = char;
            } else if char.is_digit(10) {
                second_num = char;
            }
        }
        // println!("{} Number: {}",ii,  number);
        line_sum.push(first_num);
        if second_num != 'z' {
            line_sum.push(second_num);
        } else {
            line_sum.push(first_num);
        }
        println!("{} Line Sum: {}", ii, line_sum);
        total_sum += line_sum.parse::<i32>().unwrap();
        ii += 1;
    }

    println!("Total Sum: {}",total_sum);

}