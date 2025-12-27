use crate::lines_from_bytes;
use crate::lines_from_comma_separated_bytes;
use crate::load_input;

pub const DAY: i32 = 10;

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let mut counter = 0;
    let machines = parse_parsed_input(lines);
    //Tests
    let test_machine =Machine{
        pack: MyByteMap { inner: vec![0,1,1,0] },
        switches: vec![
            MyByteMap{inner: vec![3]},
            MyByteMap{inner: vec![1,3]},
            MyByteMap{inner: vec![2]},
            MyByteMap{inner: vec![2,3]},
            MyByteMap{inner: vec![0,2]},
            MyByteMap{inner: vec![0,1]},
        ],
        joltages: Vec::new()};
    assert_eq!(machines[0],Machine{
        pack: MyByteMap { inner: vec![0,0,0,0,1,1] },
        switches: vec![
            MyByteMap{inner: vec![0,5]},
            MyByteMap{inner: vec![0,2]},
            MyByteMap{inner: vec![0,2,3,4,5]},
            MyByteMap{inner: vec![0,2,4,5]},
            MyByteMap{inner: vec![1,2,3,4,5]},
            MyByteMap{inner: vec![0,1,4,5]},
            MyByteMap{inner: vec![1,4,5]},
        ]
        ,joltages: Vec::new()});

    assert_eq!(2,test_machine.solve_machine());
    for m in machines{
        counter += m.solve_machine();
    }
    return counter;
}
#[derive(Debug, Default,Clone, PartialEq, Eq)]
struct Machine{
    pack: MyByteMap,
    switches: Vec<MyByteMap>,
    joltages: Vec<usize>
}
impl Machine{
    fn solve_machine(self: &Self) -> u64{
        let mut new_pack = Vec::new();
        for _i in 0..self.pack.inner.len(){
            new_pack.push(0);
        }
        let new_pack = MyByteMap{inner:new_pack};
        // 1 Switch
        for i in 0..self.switches.len(){
            if new_pack.clone() ^ self.switches[i].clone() == self.pack{return 1;}
        }
        // 2 Switches
        if self.switches.len() == 2{return 2;}
        for i in 0..self.switches.len(){
            for j in i+1..self.switches.len(){
                let x1 = new_pack.clone() ^self.switches[i].clone();
                let x2 = x1.clone()^self.switches[j].clone();
                if  x2  == self.pack{return 2;}
            }
        }
        // 3 Switches
        if self.switches.len() == 3{return 3;}
        for i in 0..self.switches.len(){
            for j in i+1..self.switches.len(){
                for k in j+1..self.switches.len(){
                    if ((new_pack.clone() ^
                        self.switches[i].clone()) ^
                        self.switches[j].clone()) ^
                        self.switches[k].clone()== self.pack{return 3;}
                }
            }
        }
        // 4 Switches
        if self.switches.len() == 4{return 4;}
        for i in 0..self.switches.len(){
            for j in i+1..self.switches.len(){
                for k in j+1..self.switches.len(){
                    for l in k+1..self.switches.len(){
                        if (((new_pack.clone() ^
                            self.switches[i].clone()) ^
                            self.switches[j].clone()) ^
                            self.switches[k].clone()) ^
                            self.switches[l].clone() == self.pack{return 4;}
                    }
                }
            }
        }
        // 5 Switches
        if self.switches.len() == 5{return 5;}
        for i in 0..self.switches.len(){
            for j in i+1..self.switches.len(){
                for k in j+1..self.switches.len(){
                    for l in k+1..self.switches.len(){
                        for m in l+1..self.switches.len(){
                        if ((((new_pack.clone() ^
                            self.switches[i].clone()) ^
                            self.switches[j].clone()) ^
                            self.switches[k].clone()) ^
                            self.switches[l].clone()) ^
                            self.switches[m].clone() == self.pack{return 5;}
                        }
                    }
                }
            }
        }
        // 6 Switches
        if self.switches.len() == 6{return 6;}
        for i in 0..self.switches.len(){
            for j in i+1..self.switches.len(){
                for k in j+1..self.switches.len(){
                    for l in k+1..self.switches.len(){
                        for m in l+1..self.switches.len(){
                            for n in m+1..self.switches.len(){
                                if (((((new_pack.clone() ^
                                    self.switches[i].clone()) ^
                                    self.switches[j].clone()) ^
                                    self.switches[k].clone()) ^
                                    self.switches[l].clone()) ^
                                    self.switches[m].clone()) ^
                                    self.switches[n].clone() == self.pack{return 6;}
                            }
                        }
                    }
                }
            }
        }
        // 7 Switches
        if self.switches.len() == 7{return 7;}
        for i in 0..self.switches.len(){
            for j in i+1..self.switches.len(){
                for k in j+1..self.switches.len(){
                    for l in k+1..self.switches.len(){
                        for m in l+1..self.switches.len(){
                            for n in m+1..self.switches.len(){
                                for o in n+1..self.switches.len(){
                                    if ((((((new_pack.clone() ^
                                        self.switches[i].clone()) ^
                                        self.switches[j].clone()) ^
                                        self.switches[k].clone()) ^
                                        self.switches[l].clone()) ^
                                        self.switches[m].clone()) ^
                                        self.switches[n].clone()) ^
                                        self.switches[o].clone() == self.pack{return 7;}
                                }
                            }
                        }
                    }
                }
            }
        }
        // 8 Switches
        if self.switches.len() == 8{return 8;}
        for i in 0..self.switches.len(){
            for j in i+1..self.switches.len(){
                for k in j+1..self.switches.len(){
                    for l in k+1..self.switches.len(){
                        for m in l+1..self.switches.len(){
                            for n in m+1..self.switches.len(){
                                for o in n+1..self.switches.len(){
                                    for p in o+1..self.switches.len(){
                                        if (((((((new_pack.clone() ^
                                            self.switches[i].clone()) ^
                                            self.switches[j].clone()) ^
                                            self.switches[k].clone()) ^
                                            self.switches[l].clone()) ^
                                            self.switches[m].clone()) ^
                                            self.switches[n].clone()) ^
                                            self.switches[o].clone()) ^
                                            self.switches[p].clone() == self.pack{return 8;}

                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return 9;
    }
}
/// A bytemap is a vector of bytes
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MyByteMap{inner: Vec<u8>}

impl std::ops::BitXor for MyByteMap {
    type Output = MyByteMap;
    fn bitxor(mut self, rhs: MyByteMap) -> Self::Output {
        for i in rhs.inner{
            if self.inner[i as usize]==1{self.inner[i as usize] = 0}
            else{self.inner[i as usize] = 1}
        }
        self
    }

}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn parse_parsed_input(lines: Vec<Vec<u8>>) -> Vec<Machine>{
    let mut machines = Vec::new();
    for line in lines{
        let _joltages: Vec<usize> = Vec::new();
        let mut pack = Vec::new();
        let mut switches: Vec<MyByteMap> = Vec::new();

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
                let mut mybytemap = MyByteMap{inner:Vec::new()};
                for n in numbers{
                    mybytemap.inner.push(n[0]-b'0');
                }
                switches.push(mybytemap);
            }
        }
        machines.push(Machine { pack: MyByteMap { inner: pack }, switches, joltages: _joltages })

    }
    machines
}