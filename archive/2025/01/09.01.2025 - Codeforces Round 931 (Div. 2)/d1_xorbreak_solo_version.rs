//{"name":"D1. XOR Break — Solo Version","group":"Codeforces - Codeforces Round 931 (Div. 2)","url":"https://codeforces.com/contest/1934/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n7 3\n4 2\n481885160128643072 45035996273704960\n","output":"1\n7 3\n-1\n3\n481885160128643072 337769972052787200 49539595901075456 45035996273704960\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1XORBreakSoloVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let m = input.read_long();
    let mut i = 62;
    while i >= 0 && ((n >> i) & 1) == 0 {
        i -= 1;
    }
    if ((m >> i) & 1) == 1 {
        out.print_line(1);
        out.print_line(format!("{n} {m}"));
    } else {
        i -= 1;
        while i >= 0 && ((n >> i) & 1) == 0 {
            i -= 1;
        }
        let tmp = (1 << (i + 1)) - 1;
        if m > tmp {
            out.print_line(-1);
        } else if m == tmp {
            out.print_line(1);
            out.print_line(format!("{n} {m}"));
        } else {
            out.print_line(2);
            out.print_line(format!("{n} {tmp} {m}"));
        }
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
