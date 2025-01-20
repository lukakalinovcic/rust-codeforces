//{"name":"E. Klee's SUPER DUPER LARGE Array!!!","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 2\n7 2\n5 3\n1000000000 1000000000\n","output":"1\n5\n1\n347369930\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKleesSUPERDUPERLARGEArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn sum(lo: i64, k: i64) -> i64 {
    let hi = lo + k - 1;
    (lo + hi) * k / 2
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let k = input.read_long();
    let mut lo = 0;
    let mut hi = n;
    let mut result = INF;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let sum_left = sum(k, mid);
        let sum_right = sum(k + mid, n - mid);
        let diff = sum_left - sum_right;
        result = result.min(diff.abs());
        if diff < 0 {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    out.print_line(result);
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
