//{"name":"G. Snakes","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"3 6\n1 +\n1 -\n3 +\n3 -\n2 +\n2 -\n","output":"4\n"},{"input":"5 13\n5 +\n3 +\n5 -\n2 +\n4 +\n3 +\n5 +\n5 -\n2 +\n3 -\n3 +\n3 -\n2 +\n","output":"11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSnakes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

#[derive(Clone, Copy)]
struct Sim {
    first_lo: i32,
    first_hi: i32,
    second_lo: i32,
    second_hi: i32,
    start_dist: i32,
}

impl Sim {
    fn new() -> Self {
        Sim {
            first_lo: 1,
            first_hi: 1,
            second_lo: 2,
            second_hi: 2,
            start_dist: 1,
        }
    }

    fn move_first(&mut self, op: char) {
        if op == '-' {
            self.first_lo += 1;
        } else {
            self.first_hi += 1;
            if self.first_hi == self.second_lo {
                self.second_lo += 1;
                self.second_hi += 1;
                self.start_dist += 1;
            }
        }
    }

    fn move_second(&mut self, op: char) {
        if op == '-' {
            self.second_lo += 1;
        } else {
            self.second_hi += 1;
        }
    }

    fn start_dist(&self) -> i32 {
        self.start_dist
    }

    fn first_extent(&self) -> i32 {
        self.first_hi
    }
}

fn rec(
    prev: usize,
    mask: usize,
    n: usize,
    sims: &[[Sim; 20]; 20],
    memo: &mut [[i32; 1 << 20]; 20],
) -> i32 {
    if mask == (1 << n) - 1 {
        return sims[prev][prev].first_extent();
    }
    if memo[prev][mask] != 0 {
        return memo[prev][mask];
    }
    let mut result = 1000000000;
    for next in 0..n {
        if ((mask >> next) & 1) == 1 {
            continue;
        }
        let offset = sims[prev][next].start_dist();
        let rest = rec(next, mask | (1 << next), n, sims, memo);
        result = result.min(offset + rest);
    }
    memo[prev][mask] = result;
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, q) = (input.read_int() as usize, input.read_int() as usize);

    let mut sims = [[Sim::new(); 20]; 20];
    for _ in 0..q {
        let (snake, op) = (input.read_int() as usize, input.read_char());

        for other in 0..n {
            sims[snake - 1][other].move_first(op);
            sims[other][snake - 1].move_second(op);
        }
    }

    let mut memo = [[0; 1 << 20]; 20];
    let mut result = 1000000000;
    for first in 0..n {
        result = result.min(rec(first, 1 << first, n, &sims, &mut memo));
    }
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    // println!("20 200000");
    // for i in 0..200000 {
    //     let snake = (i / 2) % 20 + 1;
    //     let op = if i % 2 == 0 { '+' } else { '-' };
    //     println!("{snake} {op}");
    // }
    tester::run_tests();
}
//END MAIN
