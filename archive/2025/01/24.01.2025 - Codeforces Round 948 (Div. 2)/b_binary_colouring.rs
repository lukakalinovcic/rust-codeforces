//{"name":"B. Binary Colouring","group":"Codeforces - Codeforces Round 948 (Div. 2)","url":"https://codeforces.com/contest/1977/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1\n14\n24\n15\n27\n11\n19\n","output":"1\n1\n5\n0 -1 0 0 1\n6\n0 0 0 -1 0 1\n5\n-1 0 0 0 1\n6\n-1 0 -1 0 0 1\n5\n-1 0 -1 0 1\n5\n-1 0 1 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBinaryColouring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();

    let mut a = vec![0; 32];
    for i in 0..32 {
        a[i] = (n >> i) & 1;
    }
    let mut i: i32 = 30;
    while i >= 0 {
        let ii = i as usize;
        if a[ii] == 0 || a[ii + 1] == 0 {
            i -= 1;
        } else if a[ii + 1] == -1 {
            a[ii + 1] = 0;
            a[ii] = -1;
            i -= 1;
        } else {
            a[ii + 2] = 1;
            a[ii + 1] = 0;
            a[ii] = -1;
            i += 2;
        }
    }
    while a.len() > 0 && *a.last().unwrap() == 0 {
        a.pop();
    }

    out.print_line(a.len());
    out.print_iter(a.into_iter());
    out.print_line("");
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
