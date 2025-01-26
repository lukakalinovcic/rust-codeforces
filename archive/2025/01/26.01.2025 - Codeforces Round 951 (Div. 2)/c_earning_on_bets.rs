//{"name":"C. Earning on Bets","group":"Codeforces - Codeforces Round 951 (Div. 2)","url":"https://codeforces.com/contest/1979/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3\n3 2 7\n2\n3 3\n5\n5 5 5 5 5\n6\n7 9 3 17 9 13\n3\n6 3 2\n5\n9 4 6 8 3\n","output":"27 41 12\n1 1\n-1\n1989 1547 4641 819 1547 1071\n-1\n8 18 12 9 24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CEarningOnBets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input
        .read_int_vec(n)
        .into_iter()
        .map(|a| 1396755360 / a)
        .collect::<Vec<_>>();
    let mut s: i64 = 0;
    for i in 0..n {
        s += a[i] as i64;
    }
    if s >= 1396755360 {
        out.print_line("-1");
    } else {
        out.print_iter(a.into_iter());
        out.print_line("");
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
