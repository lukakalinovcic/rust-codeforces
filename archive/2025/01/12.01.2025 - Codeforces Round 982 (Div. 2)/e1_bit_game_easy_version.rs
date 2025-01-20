//{"name":"E1. Bit Game (Easy Version)","group":"Codeforces - Codeforces Round 982 (Div. 2)","url":"https://codeforces.com/contest/2027/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n2\n1 6\n10 7\n3\n10 8 15\n25 4 14\n4\n8 32 65 64\n7 45 126 94\n3\n20 40 1\n23 55 1\n5\n12345 9876 86419 8641 1\n6789 54321 7532 97532 1\n2\n20 64\n44 61\n3\n57 109 55\n69 90 85\n","output":"Bob\nBob\nBob\nBob\nBob\nAlice\nAlice\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1BitGameEasyVersion"}}}

use std::ops::Not;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

enum WeirdCase {
    Potential,
    Active,
    Avoided,
}

enum State {
    Start,
    FirstBitSet(u32, WeirdCase),
    SecondBitAllZeros(u32),
    Normal(u32, WeirdCase),
}

impl WeirdCase {
    fn next(&self, a_bit: u32, x_bit: u32) -> WeirdCase {
        match self {
            WeirdCase::Potential => {
                if a_bit == 0 {
                    if x_bit == 0 {
                        WeirdCase::Potential
                    } else {
                        WeirdCase::Active
                    }
                } else {
                    if x_bit == 0 {
                        WeirdCase::Avoided
                    } else {
                        WeirdCase::Potential
                    }
                }
            }
            WeirdCase::Active => {
                if x_bit == 0 {
                    WeirdCase::Active
                } else {
                    WeirdCase::Avoided
                }
            }
            WeirdCase::Avoided => WeirdCase::Avoided,
        }
    }
}

impl State {
    fn next(&self, a_bit: u32, x_bit: u32) -> State {
        match self {
            State::Start => {
                if a_bit == 0 {
                    State::Start
                } else {
                    if x_bit == 0 {
                        State::Normal(0, WeirdCase::Avoided)
                    } else {
                        State::FirstBitSet(1, WeirdCase::Potential)
                    }
                }
            }
            State::Normal(ones, weird_state) => {
                State::Normal(ones + x_bit, weird_state.next(a_bit, x_bit))
            }
            State::FirstBitSet(ones, weird_state) => {
                if a_bit == 0 {
                    State::FirstBitSet(ones + x_bit, weird_state.next(a_bit, x_bit))
                } else {
                    if x_bit == 0 {
                        State::SecondBitAllZeros(*ones)
                    } else {
                        State::Normal(ones + 1, weird_state.next(a_bit, x_bit))
                    }
                }
            }
            State::SecondBitAllZeros(ones) => {
                if x_bit == 0 {
                    State::SecondBitAllZeros(*ones)
                } else {
                    State::Normal(ones + 1, WeirdCase::Avoided)
                }
            }
        }
    }
}

fn compute_sg(x: u32, a: u32) -> u32 {
    let mut state = State::Start;
    for bit in (0..31).rev() {
        let x_bit = (x >> bit) & 1;
        let a_bit = (a >> bit) & 1;
        state = state.next(a_bit, x_bit);
    }
    match state {
        State::Normal(ones, weird_state) => match weird_state {
            WeirdCase::Active => 0,
            _ => ones,
        },
        State::FirstBitSet(ones, _) | State::SecondBitAllZeros(ones) => {
            if ones % 2 == 0 {
                ones - 2
            } else {
                ones
            }
        }
        State::Start => panic!("should not happen"),
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let x = input.read_int_vec(n);
    let mut sg_xor = 0;
    for i in 0..n {
        sg_xor = sg_xor ^ compute_sg(x[i] as u32, a[i] as u32);
    }
    if sg_xor == 0 {
        out.print_line("Bob");
    } else {
        out.print_line("Alice");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
