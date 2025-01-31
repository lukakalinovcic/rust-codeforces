//{"name":"B. All Pairs Segments","group":"Codeforces - Codeforces Round 975 (Div. 2)","url":"https://codeforces.com/contest/2019/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n2 2\n101 200\n2 1\n6 15\n1 2 3 5 6 7\n1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\n5 8\n254618033 265675151 461318786 557391198 848083778\n6 9 15 10 6 9 4 4294967300\n","output":"0 100\n0 0 0 0 2 0 0 0 3 0 2 0 0 0 0\n291716045 0 0 0 291716045 0 301749698 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAllPairsSegments"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let x = input.read_int_vec(n);
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for i in 0..n {
        let k = (i + 1) as i64 * (n - i - 1) as i64 + i as i64;
        *cnt.entry(k).or_default() += 1;

        if i + 1 < n {
            let k = (i + 1) as i64 * (n - i - 1) as i64;
            *cnt.entry(k).or_default() += x[i + 1] - x[i] - 1;
        }
    }
    out.print_iter(
        input
            .read_long_vec(q)
            .into_iter()
            .map(|k| cnt.get(&k).cloned().unwrap_or_default()),
    );
    out.print_line("");
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
