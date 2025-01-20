//{"name":"E2. Bit Game (Hard Version)","group":"Codeforces - Codeforces Round 982 (Div. 2)","url":"https://codeforces.com/contest/2027/problem/E2","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3\n1 2 3\n3 2 2\n1\n13\n45\n5\n5 4 7 8 6\n4 4 5 5 5\n4\n6 4 8 8\n12 13 14 12\n3\n92856133 46637598 12345678\n29384774 73775896 87654321\n2\n65 12\n110 31\n4\n677810235 275091182 428565855 720629731\n74522416 889934149 3394714 230851724\n","output":"4\n4\n0\n6552\n722019507\n541\n665443265\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2BitGameHardVersion"}}}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: u32 = 1000000007;

fn mult(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) % MOD as u64) as u32
}

enum WeirdCase {
    Potential,
    Active,
    Avoided,
}

enum State {
    Start,
    FirstBitSet(WeirdCase),
    SecondBitAllZeros,
    Normal(WeirdCase),
}

impl WeirdCase {
    fn id(&self) -> u32 {
        match self {
            WeirdCase::Potential => 0,
            WeirdCase::Active => 1,
            WeirdCase::Avoided => 2,
        }
    }
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
    fn id(&self) -> u32 {
        match self {
            State::Start => 0,
            State::Normal(weird_state) => 1 + weird_state.id(),
            State::FirstBitSet(weird_state) => 4 + weird_state.id(),
            State::SecondBitAllZeros => 7,
        }
    }

    fn next(&self, a_bit: u32, x_bit: u32) -> State {
        match self {
            State::Start => {
                if a_bit == 0 {
                    State::Start
                } else {
                    if x_bit == 0 {
                        State::Normal(WeirdCase::Avoided)
                    } else {
                        State::FirstBitSet(WeirdCase::Potential)
                    }
                }
            }
            State::Normal(weird_state) => State::Normal(weird_state.next(a_bit, x_bit)),
            State::FirstBitSet(weird_state) => {
                if a_bit == 0 {
                    State::FirstBitSet(weird_state.next(a_bit, x_bit))
                } else {
                    if x_bit == 0 {
                        State::SecondBitAllZeros
                    } else {
                        State::Normal(weird_state.next(a_bit, x_bit))
                    }
                }
            }
            State::SecondBitAllZeros => {
                if x_bit == 0 {
                    State::SecondBitAllZeros
                } else {
                    State::Normal(WeirdCase::Avoided)
                }
            }
        }
    }
}

fn ways_for_sg(
    a: u32,
    target_sg: u32,
    hi: u32,
    bit: i32,
    ones: u32,
    state: State,
    memo: &mut [(u32, u32)],
) -> u32 {
    if bit == -1 {
        let sg = match state {
            State::Normal(weird_state) => match weird_state {
                WeirdCase::Active => 0,
                _ => ones,
            },
            State::FirstBitSet(_) | State::SecondBitAllZeros => {
                if ones % 2 == 0 {
                    ones - 2
                } else {
                    ones
                }
            }
            State::Start => panic!("should not happen"),
        };
        return if sg == target_sg { 1 } else { 0 };
    }
    let a_bit = (a >> bit) & 1;
    let hi_bit = (hi >> bit) & 1;
    let memo_key = ((bit as u32) * 61 + target_sg + 30 - ones) * 64
        + (state.id()
            + 8 * ((ones % 2)
                + if target_sg > 0 { 2 } else { 0 }
                + if hi == (1 << (bit + 1)) - 1 { 4 } else { 0 }));
    {
        let tmp = &mut memo[memo_key as usize];
        if tmp.0 == a {
            return tmp.1;
        }
    }
    let result = if hi_bit == 0 {
        ways_for_sg(a, target_sg, hi, bit - 1, ones, state.next(a_bit, 0), memo)
    } else {
        let zero_state = state.next(a_bit, 0);
        let one_state = state.next(a_bit, 1);
        ways_for_sg(
            a,
            target_sg,
            (1 << bit) - 1,
            bit - 1,
            ones,
            zero_state,
            memo,
        ) + ways_for_sg(
            a,
            target_sg,
            hi - (1 << bit),
            bit - 1,
            match one_state {
                State::Start => ones,
                _ => ones + 1,
            },
            one_state,
            memo,
        )
    };
    memo[memo_key as usize] = (a, result);
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut sg_xor_ways = vec![0; 32];
    sg_xor_ways[0] = 1;
    for i in 0..n {
        let a = a[i] as u32;
        let b = b[i] as u32;
        let mut next_sg_xor_cnt = vec![0; 32];
        let mut memo = vec![(0, 0); 30 * 61 * 64];
        for sg in 0..=30 {
            let sg_ways =
                ways_for_sg(a, sg, b, 29, 0, State::Start, &mut memo) - if sg == 0 { 1 } else { 0 };
            for sg_xor in 0..32 {
                let new_xor = sg_xor ^ sg;
                next_sg_xor_cnt[new_xor as usize] += mult(sg_xor_ways[sg_xor as usize], sg_ways);
                next_sg_xor_cnt[new_xor as usize] %= MOD;
            }
        }
        sg_xor_ways = next_sg_xor_cnt;
    }
    out.print_line(sg_xor_ways[0]);
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
