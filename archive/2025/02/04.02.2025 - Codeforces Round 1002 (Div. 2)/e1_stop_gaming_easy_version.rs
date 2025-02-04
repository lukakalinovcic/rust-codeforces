//{"name":"E1. Stop Gaming (Easy Version)","group":"Codeforces - Codeforces Round 1002 (Div. 2)","url":"https://codeforces.com/contest/2059/problem/E1","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2 2\n2 6\n3 4\n1 2\n7 8\n1 5\n5 4 1 2 3\n5 4 3 2 1\n3 3\n1 2 3\n4 5 6\n7 8 9\n11 1 2\n12 3 4\n13 5 6\n4 4\n1 2 3 4\n5 6 7 8\n9 10 11 12\n13 14 15 16\n17 1 2 3\n4 18 5 6\n7 19 8 20\n9 21 22 10\n","output":"3\n5\n3\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1StopGamingEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n * m);
    let b = input.read_int_vec(n * m);
    let mut result = 0;
    let mut i = 0;
    let mut j = 0;
    while j < n * m {
        let row = j / m;
        let row_start = row * m;
        if b[j] == a[i] {
            if i < row_start {
                i += 1;
                j += 1;
                continue;
            }
            let rem = (row + 1) * m - j;
            let mut ok = true;
            for k in 1..rem {
                if b[j + k] != a[i + k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                i += rem;
                j += rem;
                continue;
            }
        }
        j += 1;
        result += 1;
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
