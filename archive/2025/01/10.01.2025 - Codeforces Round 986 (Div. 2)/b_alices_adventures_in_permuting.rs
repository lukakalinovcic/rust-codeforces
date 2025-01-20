//{"name":"B. Alice's Adventures in Permuting","group":"Codeforces - Codeforces Round 986 (Div. 2)","url":"https://codeforces.com/contest/2028/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n10 1 0\n1 2 3\n100 2 1\n3 0 1\n3 0 0\n1000000000000000000 0 0\n1000000000000000000 1000000000000000000 1000000000000000000\n","output":"0\n1\n50\n2\n-1\n-1\n1000000000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAlicesAdventuresInPermuting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn doit(n: i64, b: i64, c: i64) -> i64 {
    if n == 1 {
        if c == 0 {
            0
        } else {
            1
        }
    } else if n == 2 {
        if c == 0 {
            if b == 1 {
                0
            } else {
                1
            }
        } else if c == 1 {
            1
        } else {
            2
        }
    } else {
        if b == 0 {
            if c <= n - 3 {
                -1
            } else if c <= n - 1 {
                n - 1
            } else {
                n
            }
        } else {
            let cnt_ok = ((n - 1 - c + b) / b).max(0);
            n - cnt_ok
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let b = input.read();
    let c = input.read();
    out.print_line(doit(n, b, c));
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
