//{"name":"F. Sum of Progression","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/contest/1921/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 3\n1 1 2\n1 2 2\n2 2 1\n1 1 2\n3 1\n-100000000 -100000000 -100000000\n1 1 3\n5 3\n1 2 3 4 5\n1 2 3\n2 3 2\n1 1 5\n3 1\n100000000 100000000 100000000\n1 1 3\n7 7\n34 87 5 42 -44 66 -32\n2 2 2\n4 3 1\n1 3 2\n6 2 1\n5 2 2\n2 5 2\n6 1 2\n","output":"5 1 3\n-600000000\n22 12 55\n600000000\n171 42 118 66 -108 23 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSumOfProgression"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MAX_PRECALC: usize = 100;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);
    let mut sum = vec![vec![0; n]; MAX_PRECALC];
    let mut tri = vec![vec![0; n]; MAX_PRECALC];
    for i in (0..n).rev() {
        for d in 1..=MAX_PRECALC {
            sum[d - 1][i] = a[i];
            tri[d - 1][i] = a[i];
            if i + d < n {
                sum[d - 1][i] += sum[d - 1][i + d];
                tri[d - 1][i] += tri[d - 1][i + d] + sum[d - 1][i + d];
            }
        }
    }
    let mut results = vec![];
    for _ in 0..q {
        let s = input.read_size();
        let d = input.read_size();
        let k = input.read_size();
        results.push(if d > MAX_PRECALC {
            let mut result = 0;
            for i in 0..k {
                result += a[s + i * d - 1] * (i + 1) as i64;
            }
            result
        } else {
            let mut result = tri[d - 1][s - 1];
            if s + k * d <= n {
                result -= tri[d - 1][s + k * d - 1];
                result -= sum[d - 1][s + k * d - 1] * k as i64;
            }
            result
        });
    }
    out.print_line(results);
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
