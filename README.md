My solutions to the [Advent of Code 2025](https://adventofcode.com/2025 "Aoc2025") in rust.

The total runtime is about 1 second on my machine (including parsing) and most solutions are not perfect and not written in "idiomatic" rust.

The problems were not particularly difficult except for day 10 part 2, I had to use an external crate to resolve the incomplete linear systems with up to 10 unknown variables, but I am planning to create a rust solution to resolve the problem without external crates because the good_lp crate requires a lot of time, memory and space to compile and since it is a general purpose library I could perhaps shave off some ms from the solution with a custom-made solution (it is one of the solution that takes the longest)

day1::solution() = 1092
duration = 863.929µs
day1_silver::solution() = 6616
duration = 1.345545ms
day2::solution() = 15873079081
duration = 65.420953ms
day2_silver::solution() = 22617871034
duration = 240.786916ms
day3::solution() = 17113
duration = 169.423µs
day3_silver::solution() = 169709990062889
duration = 199.52µs
day4::solution() = 1435
duration = 1.476449ms
day4_silver::solution() = 8623
duration = 28.166322ms
day5::solution() = 681
duration = 654.991µs
day5_silver::solution() = 348820208020395
duration = 361.598µs
day6::solution() = 6343365546996
duration = 293.325µs
day6_silver::solution() = 11136895955912
duration = 250.38µs
day7::solution() = 1675
duration = 173.377µs
day7_silver::solution() = 187987920774390
duration = 154.043µs
day8::solution() = 175440
duration = 44.380843ms
day8_silver::solution() = 3200955921
duration = 45.562346ms
day9::solution() = 4776487744
duration = 331.677µs
day9_silver::solution() = 1560299548
duration = 177.242848ms
day10::solution() = 500
duration = 3.09949ms
day10_silver::solution() = 19763
duration = 379.67989ms
day11::solution() = 690
duration = 527.832µs
day11_silver::solution() = 557332758684000
duration = 1.030075ms
day12::solution() = 579
duration = 538.204µs
day12_silver::solution() = 12
duration = 2.113µs
total_time_elapsed.elapsed() = 992.865229ms

