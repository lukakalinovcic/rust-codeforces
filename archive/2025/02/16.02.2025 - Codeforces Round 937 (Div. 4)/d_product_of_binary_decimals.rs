//{"name":"D. Product of Binary Decimals","group":"Codeforces - Codeforces Round 937 (Div. 4)","url":"https://codeforces.com/contest/1950/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"11\n121\n1\n14641\n12221\n10110\n100000\n99\n112\n2024\n12421\n1001\n","output":"YES\nYES\nYES\nYES\nYES\nYES\nNO\nNO\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DProductOfBinaryDecimals"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

struct PreCalc {
    result: Vec<bool>,
}

impl PreCalc {
    fn new() -> Self {
        const MAX: usize = 100000;
        let mut bin_dec = vec![];
        for mask in 1..=32 {
            let mut x = 0;
            let mut p10 = 1;
            for i in 0..=5 {
                if (mask >> i) & 1 == 1 {
                    x += p10;
                }
                p10 *= 10;
            }
            bin_dec.push(x);
        }
        let mut result = vec![false; MAX + 1];
        result[1] = true;
        for x in 2..=MAX {
            for &y in &bin_dec {
                if x % y == 0 && result[x / y] {
                    result[x] = true;
                }
            }
        }
        Self { result }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read_size();
    if precalc.result[n] {
        out.print_line("YES");
    } else {
        out.print_line("NO");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new();

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
