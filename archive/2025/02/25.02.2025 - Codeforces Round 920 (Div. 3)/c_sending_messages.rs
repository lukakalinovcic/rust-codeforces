//{"name":"C. Sending Messages","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/contest/1921/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 3 1 5\n3\n7 21 1 3\n4 6 10 13 17 20 26\n5 10 1 2\n1 2 3 4 5\n1 1000000000 1000000000 1000000000\n1000000000\n3 11 9 6\n6 8 10\n12 621526648 2585904 3566299\n51789 61859 71998 73401 247675 298086 606959 663464 735972 806043 806459 919683\n","output":"NO\nYES\nYES\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSendingMessages"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut f = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let mut prev = 0;
    for curr in input.read_long_vec(n) {
        let d = curr - prev;
        f -= b.min(d * a);
        if f <= 0 {
            out.print_line("NO");
            return;
        }
        prev = curr;
    }
    out.print_line("YES");
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
