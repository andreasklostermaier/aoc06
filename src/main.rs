use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    part_one();
    part_two();

}


fn part_one() {

    let mut current_slice: &str;
    let mut first_marker: usize = 0;

    if let Ok(lines) = read_lines("./data/testdata.txt") {

        for line in lines.flatten() {

            let stopchar = line.len() - 3;
            for slice_counter in 0..stopchar {
                current_slice = &line[slice_counter..slice_counter+4];

                if !(current_slice[1..4].contains(&current_slice[0..=0]) ||
                     current_slice[2..4].contains(&current_slice[1..=1]) ||
                     current_slice[3..=3] == current_slice[2..=2]) {
                    first_marker = slice_counter + 4;
                    break;
                }
            }
        }
    }

    // Part 1
    println!("Part 1");
    println!("The first marker is after character: {:?}", first_marker);

}

fn part_two() {

    let mut current_slice: &str;
    let mut first_marker: usize = 0;

    if let Ok(lines) = read_lines("./data/input.txt") {

        for line in lines.flatten() {

            let stopchar = line.len() - 13;
            'sliceloop: for slice_counter in 0..stopchar {
                current_slice = &line[slice_counter..slice_counter+14];

                for char_counter in 0_usize..14_usize {
                    if current_slice[char_counter+1..14].contains(&current_slice[char_counter..=char_counter]) {
                        continue 'sliceloop;
                    }
                }
                first_marker = slice_counter + 14;
                break 'sliceloop;
            }
        }
    }

    // Part 2
    println!("Part 2");
    println!("The first marker is after character: {:?}", first_marker);

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
