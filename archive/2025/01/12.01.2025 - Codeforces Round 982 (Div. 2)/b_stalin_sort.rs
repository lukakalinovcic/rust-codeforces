//{"name":"B. Stalin Sort","group":"Codeforces - Codeforces Round 982 (Div. 2)","url":"https://codeforces.com/contest/2027/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n7\n3 6 4 9 2 5 2\n5\n5 4 4 2 2\n8\n2 2 4 4 6 6 10 10\n1\n1000\n9\n6 8 9 10 12 9 7 5 4\n7\n300000000 600000000 400000000 900000000 200000000 400000000 200000000\n","output":"2\n0\n6\n0\n4\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BStalinSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);

    let mut result = n;
    for i in 0..n {
        let mut eliminate = i;
        for j in i + 1..n {
            if a[j] > a[i] {
                eliminate += 1;
            }
        }
        result = result.min(eliminate);
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
