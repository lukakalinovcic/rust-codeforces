//{"name":"D. Grid Puzzle","group":"Codeforces - Codeforces Round 960 (Div. 2)","url":"https://codeforces.com/contest/1990/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n1\n0\n4\n2 4 4 2\n4\n3 2 1 0\n3\n0 3 0\n3\n0 1 3\n3\n3 1 0\n4\n3 1 0 3\n4\n0 2 2 2\n6\n1 3 4 2 0 4\n8\n2 2 5 2 3 4 2 4\n","output":"0\n3\n2\n1\n2\n2\n3\n2\n4\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGridPuzzle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

enum TwoByTwo {
    None,
    Left,
    Right,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_unsigned_vec(n);
    let mut result = 0;
    let mut prev: TwoByTwo = TwoByTwo::None;
    for a in a {
        let (next, add) = match (prev, a) {
            (TwoByTwo::None, 0) => (TwoByTwo::None, 0),
            (TwoByTwo::None, 1..=2) => (TwoByTwo::Left, 1),
            (TwoByTwo::None, 3..) => (TwoByTwo::None, 1),
            (TwoByTwo::Left, 0..=2) => (TwoByTwo::None, 0),
            (TwoByTwo::Left, 3..=4) => (TwoByTwo::Right, 1),
            (TwoByTwo::Left, 5..) => (TwoByTwo::None, 1),
            (TwoByTwo::Right, 0) => (TwoByTwo::None, 0),
            (TwoByTwo::Right, 1..=4) => (TwoByTwo::Left, 1),
            (TwoByTwo::Right, 5..) => (TwoByTwo::None, 1),
        };
        prev = next;
        result += add;
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
