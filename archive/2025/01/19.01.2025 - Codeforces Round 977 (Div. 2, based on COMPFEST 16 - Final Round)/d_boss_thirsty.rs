//{"name":"D. Boss, Thirsty","group":"Codeforces - Codeforces Round 977 (Div. 2, based on COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n3 6\n79 20 49 5 -1000 500\n-105 9 109 24 -98 -499\n14 47 12 39 23 50\n","output":"475\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBossThirsty"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();

    let a = input.read_long_vec(m);

    let mut dp_r = vec![-INF; m + 1];
    let mut prefix = 0;
    let mut best_prefix = 0;
    for i in 1..=m {
        prefix += a[i - 1];
        dp_r[i] = prefix + best_prefix;

        best_prefix = best_prefix.max(-prefix);
    }
    let mut dp_l = vec![-INF; m + 1];
    let mut suffix = 0;
    let mut best_suffix = 0;
    for i in (0..m).rev() {
        suffix += a[i];
        dp_l[i] = suffix + best_suffix;

        best_suffix = best_suffix.max(-suffix);
    }

    for _ in 1..n {
        let a = input.read_long_vec(m);

        let mut next_dp_r = vec![-INF; m + 1];
        let mut prefix = 0;
        let mut best_prefix = 0;
        let mut best_dp_point_with_prefix = -INF;
        for i in 1..=m {
            prefix += a[i - 1];
            next_dp_r[i] = prefix + best_dp_point_with_prefix;
            let dp_point = dp_l[i].max(dp_r[i]);
            best_dp_point_with_prefix = best_dp_point_with_prefix.max(dp_point + best_prefix);
            best_prefix = best_prefix.max(-prefix);
        }

        let mut next_dp_l = vec![-INF; m + 1];
        let mut suffix = 0;
        let mut best_suffix = 0;
        let mut best_dp_point_with_suffix = -INF;
        for i in (0..m).rev() {
            suffix += a[i];
            next_dp_l[i] = suffix + best_dp_point_with_suffix;
            let dp_point = dp_l[i].max(dp_r[i]);
            best_dp_point_with_suffix = best_dp_point_with_suffix.max(dp_point + best_suffix);
            best_suffix = best_suffix.max(-suffix);
        }

        dp_l = next_dp_l;
        dp_r = next_dp_r;
    }
    let mut best = -INF;
    for i in 0..=m {
        best = best.max(dp_l[i]);
        best = best.max(dp_r[i]);
    }
    out.print_line(best);
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
