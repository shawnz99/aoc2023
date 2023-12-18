
use std::fs::read_to_string;
use std::collections::HashMap;
use std::env;

fn main() -> std::io::Result<()> {
    let file_path = "inputs/day2in.txt";
    println!("In file {:?}", file_path);
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());


    let mut win_sum = 0;


    // let mut ii = 1;
    for line in read_to_string(file_path).unwrap().lines() {
        let mut map = HashMap::new();
        let mut game = line.split(":");
        let game_num = game.nth(0).unwrap().split(" ").nth(1).unwrap();

        // println!("{}", game_num);
        for part in game.last().unwrap().split(";") {
            for cubes in part.split(",") {
                let color_num: Vec<&str> = cubes.split_whitespace().collect();
                let color = color_num[1].to_string();
                let count = color_num[0].to_string();

                if !map.contains_key(&color) {
                    // println!("Inserting {color}: {count}");
                    map.insert(color, count);
                } else {
                    // Why isn't this updating with the bigger values
                    if let Some(cur_count) = map.get_mut(&color) {
                        // println!("old {color} count: {cur_count}");
                        // println!("new potential {color} count: {count} \n");
                        if cur_count.parse::<i32>().unwrap() < count.parse::<i32>().unwrap() {
                            // println!("New is bigger than old");
                            *cur_count = count;
                        }
                        // println!("  Set {color} count: {cur_count}\n");
                    }
                }
            }
        }
        println!("{game_num}");
        let red_count: i32= map.get("red").unwrap().parse::<i32>().unwrap();
        let blue_count: i32 = map.get("blue").unwrap().parse::<i32>().unwrap();
        let green_count: i32 = map.get("green").unwrap().parse::<i32>().unwrap();

        let game_power = red_count * blue_count * green_count;

        win_sum += game_power;

        dbg!(map);
    }
    println!{"Win Sum: {win_sum}"};

    Ok(())

    // Count the largest number of cubes for each color
    // Multiply them together
    // Sum

}