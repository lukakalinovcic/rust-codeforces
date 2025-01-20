//{"name":"A. Quintomania","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n2\n114 109\n2\n17 10\n3\n76 83 88\n8\n38 45 38 80 85 92 99 106\n5\n63 58 65 58 65\n8\n117 124 48 53 48 43 54 49\n5\n95 102 107 114 121\n10\n72 77 82 75 70 75 68 75 68 75\n","output":"YES\nYES\nYES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AQuintomania"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    for (x, y) in a.iter().zip(a.iter().skip(1)) {
        let (x, y) = (*x, *y);
        let interval = (x - y).abs();
        if interval != 5 && interval != 7 {
            out.print_line("NO");
            return;
        }
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
