//{"name":"B. pspspsps","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n4\ns.sp\n6\npss..s\n5\nppppp\n2\nsp\n4\n.sp.\n8\npsss....\n1\n.\n8\npspspsps\n20\n....................\n","output":"YES\nNO\nYES\nYES\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPspspsps"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int() as usize;
    let s = input.next_token().unwrap();
    let mut first_p = None;
    let mut last_s = None;
    for i in 0..n {
        if s[i] == b's' {
            last_s = Some(i);
        } else if s[i] == b'p' && first_p.is_none() {
            first_p = Some(i);
        }
    }
    match (first_p, last_s) {
        (Some(first_p), Some(last_s)) => {
            if first_p == n - 1 || last_s == 0 {
                out.print_line("YES");
            } else {
                out.print_line("NO");
            }
        }
        _ => {
            out.print_line("YES");
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
