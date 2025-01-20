//{"name":"C. Squaring","group":"Codeforces - Codeforces Round 961 (Div. 2)","url":"https://codeforces.com/contest/1995/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3\n1 2 3\n2\n3 2\n3\n3 1 5\n4\n1 1 2 3\n3\n4 3 2\n9\n16 2 4 2 256 2 4 2 8\n11\n10010 10009 10008 10007 10006 10005 10004 10003 10002 10001 10000\n","output":"0\n1\n-1\n0\n3\n15\n55\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSquaring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let mut result: i64 = 0;
    let mut prev = (a[0], 0);
    for x in a.into_iter().skip(1) {
        let (prev_val, prev_extra) = prev;
        if x == 1 && prev.0 > 1 {
            out.print_line(-1);
            return;
        }
        let mut curr_val = x;
        while curr_val < prev_val && curr_val <= 1000000000 {
            curr_val *= curr_val;
            result += 1;
        }
        let curr_extra = prev_extra + if curr_val < prev_val { 1 } else { 0 };
        result += curr_extra;

        prev = (curr_val, curr_extra);
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
