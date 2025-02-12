//{"name":"C. Minimizing the Sum","group":"Codeforces - Educational Codeforces Round 165 (Rated for Div. 2)","url":"https://codeforces.com/contest/1969/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 1\n3 1 2\n1 3\n5\n4 2\n2 2 1 3\n6 3\n4 1 2 2 4 3\n","output":"4\n5\n5\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMinimizingTheSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);
    let mut dp = vec![vec![0; k + 1]; k + 2];
    for i in 0..n {
        let curr = i % (k + 2);
        let prev = (i + k + 1) % (k + 2);
        for j in 0..=k {
            dp[curr][j] = dp[prev][j] + a[i] as i64;
        }
        let mut mini = a[i];
        for l in 1..=k.min(i) {
            mini = mini.min(a[i - l]);
            let prev = (i + k + 1 - l) % (k + 2);
            for j in l..=k {
                dp[curr][j] = dp[curr][j].min(dp[prev][j - l] + (l + 1) as i64 * mini as i64);
            }
        }
    }
    let last = (n - 1) % (k + 2);
    out.print_line(dp[last].drain(..).min().unwrap());
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
