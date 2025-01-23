//{"name":"D. Manhattan Circle","group":"Codeforces - Codeforces Round 952 (Div. 4)","url":"https://codeforces.com/contest/1985/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5 5\n.....\n.....\n..#..\n.....\n.....\n5 5\n..#..\n.###.\n#####\n.###.\n..#..\n5 6\n......\n......\n.#....\n###...\n.#....\n1 1\n#\n5 6\n...#..\n..###.\n.#####\n..###.\n...#..\n2 10\n..........\n...#......\n","output":"3 3\n3 3\n4 2\n1 1\n3 4\n2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DManhattanCircle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut first = None;
    let mut last = None;
    for r in 0..n {
        let s = input.read_line().into_bytes();
        for c in 0..m {
            if s[c] == b'#' {
                if first.is_none() {
                    first = Some((r, c));
                }
                last = Some((r, c));
            }
        }
    }
    let first = first.unwrap();
    let last = last.unwrap();
    out.print_line(((first.0 + last.0) / 2 + 1, first.1 + 1));
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
