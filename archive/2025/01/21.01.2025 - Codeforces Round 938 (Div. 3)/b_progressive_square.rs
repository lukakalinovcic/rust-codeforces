//{"name":"B. Progressive Square","group":"Codeforces - Codeforces Round 938 (Div. 3)","url":"https://codeforces.com/contest/1955/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 2 3\n3 9 6 5 7 1 10 4 8\n3 2 3\n3 9 6 5 7 1 11 4 8\n2 100 100\n400 300 400 500\n3 2 3\n3 9 6 6 5 1 11 4 8\n4 4 4\n15 27 7 19 23 23 11 15 7 3 19 23 11 15 11 15\n","output":"NO\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BProgressiveSquare"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let c = input.read_int();
    let d = input.read_int();
    let mut b = input.read_int_vec(n * n);
    b.sort();
    let a00 = b[0];
    let mut a = vec![];
    for i in 0..n {
        for j in 0..n {
            a.push(a00 + i as i32 * d + j as i32 * c);
        }
    }
    a.sort();
    if a == b {
        out.print_line("YES");
    } else {
        out.print_line("NO");
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
