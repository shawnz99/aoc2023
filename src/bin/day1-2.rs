use std::fs::read_to_string;
use std::env;

fn main() -> std::io::Result<()> {
    let file_path = "inputs/day1in.txt";
    println!("In file {:?}", file_path);
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());



    let mut total_sum = 0;
    let mut ii = 1;
    for line in read_to_string(file_path).unwrap().lines() {
        let mut v: Vec<_> = line.match_indices("zero").collect();
        v.append(&mut line.match_indices("one").collect());
        v.append(&mut line.match_indices("two").collect());
        v.append(&mut line.match_indices("three").collect());
        v.append(&mut line.match_indices("four").collect());
        v.append(&mut line.match_indices("five").collect());
        v.append(&mut line.match_indices("six").collect());
        v.append(&mut line.match_indices("seven").collect());
        v.append(&mut line.match_indices("eight").collect());
        v.append(&mut line.match_indices("nine").collect());
        v.append(&mut line.match_indices("0").collect());
        v.append(&mut line.match_indices("1").collect());
        v.append(&mut line.match_indices("2").collect());
        v.append(&mut line.match_indices("3").collect());
        v.append(&mut line.match_indices("4").collect());
        v.append(&mut line.match_indices("5").collect());
        v.append(&mut line.match_indices("6").collect());
        v.append(&mut line.match_indices("7").collect());
        v.append(&mut line.match_indices("8").collect());
        v.append(&mut line.match_indices("9").collect());

        let low_i = v.clone().into_iter().fold((line.len()+1,""), |acc, x| {
            if x.0 < acc.0 {
                (x.0, x.1)
            } else {
                acc
            }
        });
        let high_i = v.clone().into_iter().fold((0,""), |acc, x| {
            if x.0 >= acc.0 {
                (x.0, x.1)
            } else {
                acc
            }
        });

        let x: String = get_val(low_i.1, high_i.1).chars().filter(|c| !c.is_whitespace()).collect();
        let y = x.parse::<i32>().unwrap();
        println!("{} line sum: {}", ii, y);
        total_sum += x.parse::<i32>().unwrap();

        ii += 1;
    }

    println!("Total Sum: {}", total_sum);
    Ok(())

}
fn the_val(v: &str) -> char {
    match v {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        "zero" => '0',
        "0" => '0',
        "1" => '1',
        "2" => '2',
        "3" => '3',
        "4" => '4',
        "5" => '5',
        "6" => '6',
        "7" => '7',
        "8" => '8',
        "9" => '9',
        _ => ' '
    }

}

fn get_val<'a> (low: &'a str, high: &'a str) -> String {
    let mut l = String::from(the_val(low));
    let h = the_val(high);

    l.push(h);
    l
}
