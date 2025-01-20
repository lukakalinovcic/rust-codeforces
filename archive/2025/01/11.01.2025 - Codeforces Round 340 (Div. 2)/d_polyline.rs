//{"name":"D. Polyline","group":"Codeforces - Codeforces Round 340 (Div. 2)","url":"https://codeforces.com/contest/617/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"1 -1\n1 1\n1 2\n","output":"1\n"},{"input":"-1 -1\n-1 3\n4 3\n","output":"2\n"},{"input":"1 1\n2 3\n3 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPolyline"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut p = input.read_int_pair_vec(3);
    let mut result = 3;
    for _flip_axis in 0..2 {
        for i in 0..3 {
            let (ax, ay) = p[i];
            for j in 0..3 {
                let (bx, by) = p[j];
                if i == j {
                    continue;
                }
                if ax != bx {
                    continue;
                }
                if ay > by {
                    continue;
                }
                let (cx, cy) = p[3 - i - j];
                if cx == ax {
                    result = result.min(1);
                } else if cy <= ay || cy >= by {
                    result = result.min(2);
                }
            }
        }

        p = p.into_iter().map(|(x, y)| (y, x)).collect();
    }
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
