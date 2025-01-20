//{"name":"A. Alyona and a Square Jigsaw Puzzle","group":"Codeforces - Codeforces Round 990 (Div. 2)","url":"https://codeforces.com/contest/2047/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n1\n2\n1 8\n5\n1 3 2 1 2\n7\n1 2 1 10 2 7 2\n14\n1 10 10 100 1 1 10 1 10 2 10 2 10 1\n","output":"1\n2\n2\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAlyonaAndASquareJigsawPuzzle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn is_odd_square(n: i64) -> bool {
    let mut lo = 1;
    let mut hi = n;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let mid_sq = mid * mid;
        if mid_sq == n {
            return mid % 2 == 1;
        } else if mid_sq < n {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    return false;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut sum = 0;
    let mut result = 0;
    for x in a {
        sum += x;
        if is_odd_square(sum as i64) {
            result += 1;
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
