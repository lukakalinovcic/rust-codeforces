//{"name":"C. Everything Nim","group":"Codeforces - Codeforces Round 941 (Div. 2)","url":"https://codeforces.com/contest/1966/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5\n3 3 3 3 3\n2\n1 7\n7\n1 3 9 7 4 2 100\n3\n1 2 3\n6\n2 1 3 4 2 4\n8\n5 7 2 9 6 3 3 2\n1\n1000000000\n","output":"Alice\nBob\nAlice\nAlice\nBob\nAlice\nAlice\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CEverythingNim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);
    a.sort();
    a.dedup();
    let n = a.len();
    for i in (1..n).rev() {
        a[i] -= a[i - 1];
    }
    let mut winning = false;
    for x in a.into_iter().rev() {
        if !winning {
            winning = true;
        } else {
            winning = x >= 2;
        }
    }
    if winning {
        out.print_line("Alice");
    } else {
        out.print_line("Bob");
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
