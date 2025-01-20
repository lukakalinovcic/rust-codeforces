//{"name":"B. Maximize Mex","group":"Codeforces - Codeforces Round 977 (Div. 2, based on COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 3\n0 3 2 1 5 2\n6 2\n1 3 4 1 0 2\n4 5\n2 5 10 3\n","output":"4\n6\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaximizeMex"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut x = input.read();
    if x > n {
        x = n;
    }
    let mut a = input.read_int_vec(n);
    a.sort();
    let mut cnt = vec![0; x];
    let mut j = 0;
    for i in 0..n {
        while j < n && a[j] as usize <= i {
            cnt[a[j] as usize % x] += 1;
            j += 1;
        }
        if cnt[i % x] > 0 {
            cnt[i % x] -= 1;
        } else {
            out.print_line(i);
            return;
        }
    }
    out.print_line(n);
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
