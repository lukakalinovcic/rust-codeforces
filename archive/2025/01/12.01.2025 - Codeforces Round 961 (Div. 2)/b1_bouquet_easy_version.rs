//{"name":"B1. Bouquet (Easy Version)","group":"Codeforces - Codeforces Round 961 (Div. 2)","url":"https://codeforces.com/contest/1995/problem/B1","interactive":false,"timeLimit":1500,"tests":[{"input":"5\n5 10\n1 1 2 2 3\n8 20\n4 2 7 5 6 1 1 1\n8 100000\n239 30 610 122 24 40 8 2\n11 13\n2 4 11 1 1 2 3 5 4 3 2\n8 1033\n206 206 206 207 207 207 207 1000\n","output":"7\n13\n610\n13\n1033\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1BouquetEasyVersion"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve1(m: i64, a: i64, c: i64) -> i64 {
    let k = c.min(m / a);
    k * a
}

fn solve2(m: i64, a: i64, c1: i64, c0: i64) -> i64 {
    let k0 = c0.min(m / (a - 1));
    let k1 = c1.min((m - k0 * (a - 1)) / a);

    let rm = m - k0 * (a - 1) - k1 * a;
    let adjust = rm.min(k0).min(c1 - k1);
    let k0 = k0 - adjust;
    let k1 = k1 + adjust;
    k0 * (a - 1) + k1 * a
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read_long();
    let a = input.read_long_vec(n);
    let mut c = HashMap::new();
    for x in a {
        let entry = c.entry(x).or_default();
        *entry = *entry + 1;
    }
    let mut result = 0;
    for (x, v) in c.iter() {
        let (a1, c1) = (*x, *v);
        result = result.max(solve1(m, a1, c1));
        if let Some(c0) = c.get(&(a1 - 1)) {
            let c0 = *c0;
            result = result.max(solve2(m, a1, c1, c0));
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
