//{"name":"B. New Bakery","group":"Codeforces - Codeforces Round 953 (Div. 2)","url":"https://codeforces.com/contest/1978/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n4 4 5\n5 5 9\n10 10 5\n5 5 11\n1000000000 1000000000 1000000000\n1000000000 1000000000 1\n1000 1 1000\n","output":"17\n35\n100\n45\n1000000000000000000\n1000000000000000000\n500500\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNewBakery"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    out.print_line(doit(n, a, b));
}

fn doit(n: i64, a: i64, b: i64) -> i64 {
    if b < a {
        n * a
    } else {
        let k = (b - a).min(n);
        (b + b - k + 1) * k / 2 + (n - k) * a
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
