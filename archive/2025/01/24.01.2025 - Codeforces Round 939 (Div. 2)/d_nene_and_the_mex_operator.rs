//{"name":"D. Nene and the Mex Operator","group":"Codeforces - Codeforces Round 939 (Div. 2)","url":"https://codeforces.com/contest/1956/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n0 1\n","output":"4 1\n1 2\n"},{"input":"3\n1 3 9\n","output":"13 0\n"},{"input":"4\n1 100 2 1\n","output":"105 2\n3 3\n3 4\n"},{"input":"1\n0\n","output":"1 1\n1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNeneAndTheMexOperator"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        dp[i + 1] = dp[i + 1].max(dp[i] + a[i]);
        for j in i + 1..=n {
            dp[j] = dp[j].max(dp[i] + (j - i) as i32 * (j - i) as i32);
        }
    }
    let mut ops = vec![];
    let mut i = n;
    while i > 0 {
        if dp[i] == dp[i - 1] + a[i - 1] {
            i -= 1;
        } else {
            let mut j = i - 1;
            while dp[i] != dp[j] + (i - j) as i32 * (i - j) as i32 {
                j -= 1;
            }
            for k in j..i {
                if a[k] != 0 {
                    ops.push((k, k));
                }
            }
            gen_steps(j, i - 1, &mut ops);
            ops.push((j, i - 1));
            i = j;
        }
    }
    out.print_line((dp[n], ops.len()));
    for (i, j) in ops {
        out.print_line((i + 1, j + 1));
    }
}

fn gen_steps(i: usize, j: usize, ops: &mut Vec<(usize, usize)>) {
    if i == j {
        return;
    }
    gen_steps(i + 1, j, ops);
    ops.push((i + 1, j));
    if i + 1 < j {
        ops.push((i + 1, j - 1));
    }
    gen_steps(i, j - 1, ops);
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
    tester::run_tests();
}
//END MAIN
