use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 4;

//Joltage... find biggest digit (that isn't last for example 14811224890: 8 on index 8) and then the biggest digit after that index (in that case 9)

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let mut lines = parse_input(input);
    let mut counter = 0;
    //Tests
    // assert_eq!(check_line("..@@.@@@@.\n".into(),None, Some("@@@.@.@.@@\n".into())), 5);
    // assert_eq!(check_line("@@@.@.@.@@\n".into(),Some("..@@.@@@@.\n".into()), Some("@@@@@.@.@@\n".into())), 1);
    // assert_eq!(check_line("@@@@@.@.@@\n".into(),Some("@@@.@.@.@@\n".into()), Some("@.@@@@..@.\n".into())), 1);
    // assert_eq!(check_line("@.@@@@..@.\n".into(),Some("@@@@@.@.@@\n".into()), Some("@@.@@@@.@@\n".into())), 0);
    // assert_eq!(check_line("@@.@@@@.@@\n".into(),Some("@.@@@@..@.\n".into()), Some(".@@@@@@@.@\n".into())), 2);
    // assert_eq!(check_line(".@@@@@@@.@\n".into(),Some("@@.@@@@.@@\n".into()), Some(".@.@.@.@@@\n".into())), 0);
    // assert_eq!(check_line(".@.@.@.@@@\n".into(),Some(".@@@@@@@.@\n".into()), Some("@.@@@.@@@@\n".into())), 0);
    // assert_eq!(check_line("@.@@@.@@@@\n".into(),Some(".@.@.@.@@@\n".into()), Some(".@@@@@@@@.\n".into())), 1);
    // assert_eq!(check_line(".@@@@@@@@.\n".into(),Some("@.@@@.@@@@\n".into()), Some("@.@.@@@.@.\n".into())), 0);
    // assert_eq!(check_line("@.@.@@@.@.\n".into(),Some(".@@@@@@@@.\n".into()), None), 3);


    let mut end = false;
    while !end{
        let mut counter2 = 0;
        let mut lines2 = Vec::new();
        for i in 0..lines.len(){
            if i == 0{
                let result = check_line(lines[i].clone(), None, Some(lines[i+1].clone()));
                counter2 += result.0;
                lines2.push(result.1);
            }
            else if i == lines.len()-1{
                let result = check_line(lines[i].clone(), Some(lines[i-1].clone()), None);
                counter2 += result.0;
                lines2.push(result.1);
            }else{
                let result = check_line(lines[i].clone(), Some(lines[i-1].clone()), Some(lines[i+1].clone()));
                counter2 += result.0;
                lines2.push(result.1);
            }
        }
        counter += counter2;
        lines = lines2;
        if counter2 == 0{
            end = true;
        }
    }


    return counter;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn check_line(line: Vec<u8>, up_line: Option<Vec<u8>>, down_line: Option<Vec<u8>>) -> (u64,Vec<u8>){
    let mut line = line.clone();
    line.pop().unwrap();
    let mut new_line = line.clone();
    let mut counter = 0;
    for x in 0..line.len(){
        //How many paper sorround line[x]
        if line[x] == b'@'{
            let mut c = 0;
            if x as i32 > 0{
                if up_line.is_some() && up_line.clone().unwrap()[x-1] == b'@'{c += 1}
                if line[x-1] == b'@'{c += 1}
                if down_line.is_some() && down_line.clone().unwrap()[x-1] == b'@'{c += 1}
            }
            if x as i32 + 1 < line.len() as i32{
                if up_line.is_some() && up_line.clone().unwrap()[x+1] == b'@'{c += 1}
                if line[x+1] == b'@'{c += 1}
                if down_line.is_some() && down_line.clone().unwrap()[x+1] == b'@'{c += 1}
            }
            if up_line.is_some() && up_line.clone().unwrap()[x] == b'@'{c += 1}
            if down_line.is_some() && down_line.clone().unwrap()[x] == b'@'{c += 1}

            if c < 4{counter+=1; new_line[x]=b'.'}
        }
    }
    new_line.push(10);
    return (counter, new_line);
}
