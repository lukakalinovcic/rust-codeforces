//{"name":"D. Speedbreaker","group":"Codeforces - Codeforces Round 975 (Div. 2)","url":"https://codeforces.com/contest/2019/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6\n6 3 3 3 5 5\n6\n5 6 4 1 4 5\n9\n8 6 4 2 1 3 5 7 9\n","output":"3\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSpeedbreaker"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut lo = vec![n; n + 1];
    let mut hi = vec![0; n + 1];
    for i in 0..n {
        let x = a[i] as usize;
        lo[x] = lo[x].min(i);
        hi[x] = hi[x].max(i);
    }
    for i in 1..=n {
        lo[i] = lo[i].min(lo[i - 1]);
        hi[i] = hi[i].max(hi[i - 1]);
        if hi[i] >= lo[i] + i {
            out.print_line(0);
            return;
        }
    }
    let mut lo = 0;
    let mut hi = n as i32 - 1;
    for i in 0..n {
        lo = lo.max(i as i32 - a[i] + 1);
        hi = hi.min(i as i32 + a[i] - 1);
    }
    out.print_line(hi - lo + 1);
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
