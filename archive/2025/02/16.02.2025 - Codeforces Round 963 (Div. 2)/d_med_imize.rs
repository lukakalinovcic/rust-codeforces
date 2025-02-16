//{"name":"D. Med-imize","group":"Codeforces - Codeforces Round 963 (Div. 2)","url":"https://codeforces.com/contest/1993/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 3\n3 9 9 2\n5 3\n3 2 5 6 4\n7 1\n5 9 2 6 5 4 6\n8 2\n7 1 2 6 8 3 4 5\n4 5\n3 4 5 6\n","output":"3\n4\n9\n6\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMedImize"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);
    let rem = ((n - 1) % k) + 1;
    let mut lo = 1;
    let mut hi = 1000000000;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if doit(&a, mid, k) >= (rem + 2) / 2 {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    out.print_line(lo);
}

fn doit(a: &[i32], target: i32, k: usize) -> usize {
    let n = a.len();
    let rem = ((n - 1) % k) + 1;
    let mut dp = vec![0; n + 1];
    for i in (0..n).rev() {
        let t = i % k;
        if t < rem {
            let curr = if a[i] >= target { 1 } else { 0 };
            if t + 1 < rem {
                dp[i] = dp[i].max(curr + dp[i + 1]);
            } else {
                dp[i] = dp[i].max(curr);
            }
            if i + k <= n {
                dp[i] = dp[i].max(dp[i + k]);
            }
        }
    }
    dp[0]
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
