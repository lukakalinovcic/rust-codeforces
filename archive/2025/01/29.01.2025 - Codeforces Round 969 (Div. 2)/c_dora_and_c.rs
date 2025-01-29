//{"name":"C. Dora and C++","group":"Codeforces - Codeforces Round 969 (Div. 2)","url":"https://codeforces.com/contest/2007/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n4 5 5\n1 3 4 4\n4 2 3\n1 3 4 6\n4 7 7\n1 1 2 6\n3 15 9\n1 9 5\n3 18 12\n1 4 5\n7 27 36\n33 13 23 12 35 24 41\n10 6 9\n15 5 6 9 8 2 12 15 3 8\n2 1 1000000000\n1 1000000000\n6 336718728 709848696\n552806726 474775724 15129785 371139304 178408298 13106071\n6 335734893 671469786\n138885253 70095920 456876775 9345665 214704906 375508929\n","output":"3\n0\n3\n2\n3\n5\n1\n0\n17\n205359241\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDoraAndC"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int();
    let b = input.read_int();
    let g = gcd(a, b);
    let c = input.read_int_vec(n);
    let mut c = c.into_iter().map(|c| c % g).collect::<Vec<_>>();
    c.sort();
    let mut result = c[c.len() - 1] - c[0];
    for i in 1..n {
        result = result.min(g - (c[i] - c[i - 1]));
    }
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
