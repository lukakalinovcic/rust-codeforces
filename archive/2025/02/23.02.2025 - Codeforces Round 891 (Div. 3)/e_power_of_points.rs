//{"name":"E. Power of Points","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 4 3\n5\n1 2 5 7 1\n4\n1 10 100 1000\n","output":"8 7 6\n16 15 18 24 16\n1111 1093 1093 2893\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPowerOfPoints"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut x = input
        .read_int_vec(n)
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<Vec<_>>();
    x.sort();
    let mut prefix = vec![0; n];
    for i in 1..n {
        prefix[i] = prefix[i - 1] + (x[i].0 - x[i - 1].0) as i64 * i as i64;
    }
    let mut suffix = 0;
    let mut results = vec![0; n];
    for i in (0..n).rev() {
        if i != n - 1 {
            suffix += (x[i + 1].0 - x[i].0) as i64 * (n - i - 1) as i64;
        }
        results[x[i].1] = n as i64 + prefix[i] + suffix;
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
