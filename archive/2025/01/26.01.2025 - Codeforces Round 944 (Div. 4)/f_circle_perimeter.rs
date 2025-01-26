//{"name":"F. Circle Perimeter","group":"Codeforces - Codeforces Round 944 (Div. 4)","url":"https://codeforces.com/contest/1971/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n2\n3\n4\n5\n1984\n","output":"8\n16\n20\n24\n40\n12504\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FCirclePerimeter"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn dst(x: i32, y: i32) -> i64 {
    let x = x as i64;
    let y = y as i64;
    x * x + y * y
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let r = input.read_int();
    let rr = r as i64;
    let rr_lo = rr * rr;
    let rr_hi = (rr + 1) * (rr + 1);
    let mut y1 = r;
    let mut y2 = r;
    let mut result = 0;
    for x in 1..=r {
        y2 = y1;
        while dst(x, y2) >= rr_hi {
            y2 -= 1;
        }
        while dst(x, y2 + 1) < rr_hi {
            y2 += 1;
        }
        while y1 - 1 > 0 && dst(x, y1 - 1) >= rr_lo {
            y1 -= 1;
        }
        result += y2 - y1 + 1;
    }
    result = (result + 1) * 4;
    out.print_line(result);
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
