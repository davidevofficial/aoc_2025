use crate::lines_from_bytes;
use crate::lines_from_comma_separated_bytes;
use crate::load_input;

pub const DAY: i32 = 10;
use good_lp::{highs, variables, variable, Expression, IntoAffineExpression, SolverModel, Solution};
/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let mut counter: f64 = 0.0;
    let machines = parse_parsed_input(lines);
    //Tests
    assert_eq!(61, machines[0].joltages[0]);
    assert_eq!(50, machines[0].joltages[5]);

    // For now I am using the good_lp crate but I am planning to implement it myself in the future
    for m in machines{
        let mut vars = variables!();
        let press_vars = (0..m.switches.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect::<Vec<_>>();

        let mut problem = highs(vars.minimise(press_vars.iter().sum::<Expression>()));
        let mut exprs = vec![0.into_expression(); m.joltages.len()];
        for i in 0..m.switches.len() {
            for &x in &m.switches[i] {
                exprs[x] += press_vars[i];
            }
        }
        for (e, j) in exprs.into_iter().zip(m.joltages) {
            problem.add_constraint(e.eq(j as f64));
        }
        let sol = problem.solve().unwrap();
        counter += press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as f64

    }
    counter as u64

}
#[derive(Debug, Default,Clone, PartialEq, Eq)]
struct Machine{
    pack: Vec<u8>,
    switches: Vec<Vec<usize>>,
    joltages: Vec<usize>
}
// impl Machine{
//     fn solve_machine(self: &Self) -> u64{
//         let mut new_pack = Vec::new();
//         for _i in 0..self.pack.inner.len(){
//             new_pack.push(0);
//         }
//         let new_pack = MyByteMap{inner:new_pack};
//         // 1 Switch
//         for i in 0..self.switches.len(){
//             if new_pack.clone() ^ self.switches[i].clone() == self.pack{return 1;}
//         }
//         // 2 Switches
//         if self.switches.len() == 2{return 2;}
//         for i in 0..self.switches.len(){
//             for j in i+1..self.switches.len(){
//                 let x1 = new_pack.clone() ^self.switches[i].clone();
//                 let x2 = x1.clone()^self.switches[j].clone();
//                 if  x2  == self.pack{return 2;}
//             }
//         }
//         // 3 Switches
//         if self.switches.len() == 3{return 3;}
//         for i in 0..self.switches.len(){
//             for j in i+1..self.switches.len(){
//                 for k in j+1..self.switches.len(){
//                     if ((new_pack.clone() ^
//                         self.switches[i].clone()) ^
//                         self.switches[j].clone()) ^
//                         self.switches[k].clone()== self.pack{return 3;}
//                 }
//             }
//         }
//         // 4 Switches
//         if self.switches.len() == 4{return 4;}
//         for i in 0..self.switches.len(){
//             for j in i+1..self.switches.len(){
//                 for k in j+1..self.switches.len(){
//                     for l in k+1..self.switches.len(){
//                         if (((new_pack.clone() ^
//                             self.switches[i].clone()) ^
//                             self.switches[j].clone()) ^
//                             self.switches[k].clone()) ^
//                             self.switches[l].clone() == self.pack{return 4;}
//                     }
//                 }
//             }
//         }
//         // 5 Switches
//         if self.switches.len() == 5{return 5;}
//         for i in 0..self.switches.len(){
//             for j in i+1..self.switches.len(){
//                 for k in j+1..self.switches.len(){
//                     for l in k+1..self.switches.len(){
//                         for m in l+1..self.switches.len(){
//                         if ((((new_pack.clone() ^
//                             self.switches[i].clone()) ^
//                             self.switches[j].clone()) ^
//                             self.switches[k].clone()) ^
//                             self.switches[l].clone()) ^
//                             self.switches[m].clone() == self.pack{return 5;}
//                         }
//                     }
//                 }
//             }
//         }
//         // 6 Switches
//         if self.switches.len() == 6{return 6;}
//         for i in 0..self.switches.len(){
//             for j in i+1..self.switches.len(){
//                 for k in j+1..self.switches.len(){
//                     for l in k+1..self.switches.len(){
//                         for m in l+1..self.switches.len(){
//                             for n in m+1..self.switches.len(){
//                                 if (((((new_pack.clone() ^
//                                     self.switches[i].clone()) ^
//                                     self.switches[j].clone()) ^
//                                     self.switches[k].clone()) ^
//                                     self.switches[l].clone()) ^
//                                     self.switches[m].clone()) ^
//                                     self.switches[n].clone() == self.pack{return 6;}
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         // 7 Switches
//         if self.switches.len() == 7{return 7;}
//         for i in 0..self.switches.len(){
//             for j in i+1..self.switches.len(){
//                 for k in j+1..self.switches.len(){
//                     for l in k+1..self.switches.len(){
//                         for m in l+1..self.switches.len(){
//                             for n in m+1..self.switches.len(){
//                                 for o in n+1..self.switches.len(){
//                                     if ((((((new_pack.clone() ^
//                                         self.switches[i].clone()) ^
//                                         self.switches[j].clone()) ^
//                                         self.switches[k].clone()) ^
//                                         self.switches[l].clone()) ^
//                                         self.switches[m].clone()) ^
//                                         self.switches[n].clone()) ^
//                                         self.switches[o].clone() == self.pack{return 7;}
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         // 8 Switches
//         if self.switches.len() == 8{return 8;}
//         for i in 0..self.switches.len(){
//             for j in i+1..self.switches.len(){
//                 for k in j+1..self.switches.len(){
//                     for l in k+1..self.switches.len(){
//                         for m in l+1..self.switches.len(){
//                             for n in m+1..self.switches.len(){
//                                 for o in n+1..self.switches.len(){
//                                     for p in o+1..self.switches.len(){
//                                         if (((((((new_pack.clone() ^
//                                             self.switches[i].clone()) ^
//                                             self.switches[j].clone()) ^
//                                             self.switches[k].clone()) ^
//                                             self.switches[l].clone()) ^
//                                             self.switches[m].clone()) ^
//                                             self.switches[n].clone()) ^
//                                             self.switches[o].clone()) ^
//                                             self.switches[p].clone() == self.pack{return 8;}

//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         return 9;
//     }
// }
/// A bytemap is a vector of bytes

/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn parse_parsed_input(lines: Vec<Vec<u8>>) -> Vec<Machine>{
    let mut machines = Vec::new();
    for line in lines{
        let mut joltages: Vec<usize> = Vec::new();
        let mut pack = Vec::new();
        let mut switches: Vec<Vec<usize>> = Vec::new();

        let mut square_bracket_idx = 0;
        for i in 0..12{
            if line[i] == b']'{
                square_bracket_idx = i;
            }
        }
        for i in 1..square_bracket_idx{
            match line[i]{
                b'#' => pack.push(1),
                b'.' => pack.push(0),
                _ => pack.push(0)
            }
        }
        let mut left_parentesis = 0;
        for i in 0..line.len(){
            if line[i] == b'('{
                left_parentesis = i;
            }
            if line[i] == b')'{
                let numbers = lines_from_comma_separated_bytes(line[left_parentesis+1..i].to_vec());
                let mut mybytemap = Vec::new();
                for n in numbers{
                    mybytemap.push((n[0]-b'0') as usize);
                }
                switches.push(mybytemap);
            }
        }
        let mut left_curly = 0;
        for i in square_bracket_idx..line.len(){
            if line[i] == b'{'{
                left_curly = i;
            }
        }
        let mut new_lines =
            lines_from_comma_separated_bytes(line[left_curly+1..line.clone().len()-1].to_vec());

        for l in new_lines{
            let from = l[0..l.len()-1].to_vec();
            let from: u64 = String::from_utf8(from)
                .map_err(|_| "Invalid UTF-8").unwrap()
                .parse()
                .map_err(|_| "Not a number").unwrap();

            joltages.push(from as usize);
        }


        machines.push(Machine { pack, switches, joltages})

    }
    machines
}