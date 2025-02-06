//{"name":"D1. Turtle and a MEX Problem (Easy Version)","group":"Codeforces - Codeforces Round 968 (Div. 2)","url":"https://codeforces.com/contest/2003/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 4\n2 0 2\n3 2 3 3\n4 7 0 1 5\n3 4\n5 0 2 0 4 11\n1 1\n5 1 3 0 3 3\n2 50\n2 1 2\n2 1 2\n1 1\n7 1 2 4 1 4 9 5\n4 114514\n2 2 2\n5 7 3 6 0 3\n3 0 1 1\n5 0 9 2 1 5\n5 1919810\n1 2\n2 324003 0\n3 1416324 2 1460728\n4 1312631 2 0 1415195\n5 1223554 192248 2 1492515 725556\n","output":"16\n20\n1281\n6\n6556785365\n1842836177961\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1TurtleAndAMEXProblemEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read_int();
    let mut top = 0;
    for _ in 0..n {
        let k = input.read();
        let (_, x) = inspect(input.read_int_vec(k));
        top = top.max(x);
    }
    let result = if m <= top {
        (m + 1) as i64 * top as i64
    } else {
        (top + 1) as i64 * top as i64 + (top + 1 + m) as i64 * (m - top) as i64 / 2
    };
    out.print_line(result);
}

fn inspect(a: Vec<i32>) -> (i32, i32) {
    let mut a = a;
    a.sort();
    let mut first = None;
    let mut second = None;
    let mut i = 0;
    for x in 0..a.len() as i32 + 2 {
        let mut missing = true;
        while i < a.len() && a[i] == x {
            i += 1;
            missing = false;
        }
        if missing {
            if first.is_none() {
                first = Some(x);
            } else if second.is_none() {
                second = Some(x);
            }
        }
    }
    (first.unwrap(), second.unwrap())
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
