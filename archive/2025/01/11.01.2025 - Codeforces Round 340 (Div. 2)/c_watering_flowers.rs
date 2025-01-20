//{"name":"C. Watering Flowers","group":"Codeforces - Codeforces Round 340 (Div. 2)","url":"https://codeforces.com/contest/617/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2 -1 0 5 3\n0 2\n5 2\n","output":"6\n"},{"input":"4 0 0 5 0\n9 4\n8 3\n-1 0\n1 4\n","output":"33\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CWateringFlowers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let (x1, y1) = (input.read_long(), input.read_long());
    let (x2, y2) = (input.read_long(), input.read_long());
    let mut d_sq = vec![];
    for (x, y) in input.read_long_pair_vec(n) {
        d_sq.push((
            (x - x1) * (x - x1) + (y - y1) * (y - y1),
            (x - x2) * (x - x2) + (y - y2) * (y - y2),
        ));
    }
    d_sq.sort();
    let mut result = INF;
    let mut mx_d2 = 0;
    for i in (0..n).rev() {
        let (d1, d2) = d_sq[i];
        result = result.min(d1 + mx_d2);
        mx_d2 = mx_d2.max(d2);
    }
    result = result.min(mx_d2);
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
