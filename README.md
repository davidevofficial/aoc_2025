My solutions to the [Advent of Code 2025](https://adventofcode.com/2025 "Aoc2025") in rust.

The total runtime is about 1 second on my machine (including parsing) and most solutions are not perfect and not written in "idiomatic" rust.

The problems were not particularly difficult except for day 10 part 2, I had to use an external crate to resolve the incomplete linear systems with up to 10 unknown variables, but I am planning to create a rust solution to resolve the problem without external crates because the good_lp crate requires a lot of time, memory and space to compile and since it is a general purpose library I could perhaps shave off some ms from the solution with a custom-made solution (it is one of the solution that takes the longest)

[src/main.rs:33:5] day1::solution() = 1092
[src/main.rs:35:5] duration = 863.929µs
[src/main.rs:37:5] day1_silver::solution() = 6616
[src/main.rs:39:5] duration = 1.345545ms
[src/main.rs:43:5] day2::solution() = 15873079081
[src/main.rs:45:5] duration = 65.420953ms
[src/main.rs:47:5] day2_silver::solution() = 22617871034
[src/main.rs:49:5] duration = 240.786916ms
[src/main.rs:53:5] day3::solution() = 17113
[src/main.rs:55:5] duration = 169.423µs
[src/main.rs:57:5] day3_silver::solution() = 169709990062889
[src/main.rs:59:5] duration = 199.52µs
[src/main.rs:63:5] day4::solution() = 1435
[src/main.rs:65:5] duration = 1.476449ms
[src/main.rs:67:5] day4_silver::solution() = 8623
[src/main.rs:69:5] duration = 28.166322ms
[src/main.rs:73:5] day5::solution() = 681
[src/main.rs:75:5] duration = 654.991µs
[src/main.rs:77:5] day5_silver::solution() = 348820208020395
[src/main.rs:79:5] duration = 361.598µs
[src/main.rs:83:5] day6::solution() = 6343365546996
[src/main.rs:85:5] duration = 293.325µs
[src/main.rs:87:5] day6_silver::solution() = 11136895955912
[src/main.rs:89:5] duration = 250.38µs
[src/main.rs:93:5] day7::solution() = 1675
[src/main.rs:95:5] duration = 173.377µs
[src/main.rs:97:5] day7_silver::solution() = 187987920774390
[src/main.rs:99:5] duration = 154.043µs
[src/main.rs:103:5] day8::solution() = 175440
[src/main.rs:105:5] duration = 44.380843ms
[src/main.rs:107:5] day8_silver::solution() = 3200955921
[src/main.rs:109:5] duration = 45.562346ms
[src/main.rs:113:5] day9::solution() = 4776487744
[src/main.rs:115:5] duration = 331.677µs
[src/main.rs:117:5] day9_silver::solution() = 1560299548
[src/main.rs:119:5] duration = 177.242848ms
[src/main.rs:123:5] day10::solution() = 500
[src/main.rs:125:5] duration = 3.09949ms
[src/main.rs:127:5] day10_silver::solution() = 19763
[src/main.rs:129:5] duration = 379.67989ms
[src/main.rs:133:5] day11::solution() = 690
[src/main.rs:135:5] duration = 527.832µs
[src/main.rs:137:5] day11_silver::solution() = 557332758684000
[src/main.rs:139:5] duration = 1.030075ms
[src/main.rs:143:5] day12::solution() = 579
[src/main.rs:145:5] duration = 538.204µs
[src/main.rs:147:5] day12_silver::solution() = 12
[src/main.rs:149:5] duration = 2.113µs
[src/main.rs:151:5] total_time_elapsed.elapsed() = 992.865229ms

