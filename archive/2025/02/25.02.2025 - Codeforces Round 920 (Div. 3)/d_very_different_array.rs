//{"name":"D. Very Different Array","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/contest/1921/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n4 6\n6 1 2 4\n3 5 1 7 2 3\n3 4\n1 1 1\n1 1 1 1\n5 5\n1 2 3 4 5\n1 2 3 4 5\n2 6\n5 8\n8 7 5 8 2 10\n2 2\n4 1\n9 6\n4 6\n8 10 6 4\n3 10 6 1 8 9\n3 5\n6 5 2\n1 7 9 7 2\n5 5\n9 10 6 3 7\n5 9 2 3 9\n1 6\n3\n2 7 10 1 1 5\n","output":"16\n0\n12\n11\n10\n23\n15\n25\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVeryDifferentArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_int_vec(n);
    let mut b = input.read_int_vec(m);
    a.sort();
    b.sort();
    let mut la = 0;
    let mut ra = (n - 1) as i32;
    let mut lb = 0;
    let mut rb = (m - 1) as i32;
    let mut result: i64 = 0;
    while la <= ra {
        let lr = (a[la as usize] - b[rb as usize]).abs();
        let rl = (a[ra as usize] - b[lb as usize]).abs();
        if lr >= rl {
            result += lr as i64;
            la += 1;
            rb -= 1;
        } else {
            result += rl as i64;
            ra -= 1;
            lb += 1;
        }
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
