//{"name":"C. Beautiful Triple Pairs","group":"Codeforces - Codeforces Round 946 (Div. 3)","url":"https://codeforces.com/contest/1974/problem/C","interactive":false,"timeLimit":4000,"tests":[{"input":"8\n5\n3 2 2 2 3\n5\n1 2 1 2 1\n8\n1 2 3 2 2 3 4 2\n4\n2 1 1 1\n8\n2 1 1 2 1 1 1 1\n7\n2 1 1 1 1 1 1\n6\n2 1 1 1 1 1\n5\n2 1 1 1 1\n","output":"2\n0\n3\n1\n8\n4\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBeautifulTriplePairs"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut cnt_abc: HashMap<_, i32> = HashMap::new();
    let mut cnt_ab: HashMap<_, i32> = HashMap::new();
    let mut cnt_bc: HashMap<_, i32> = HashMap::new();
    let mut cnt_ac: HashMap<_, i32> = HashMap::new();
    for i in 0..n - 2 {
        *cnt_abc.entry((a[i], a[i + 1], a[i + 2])).or_default() += 1;
        *cnt_ab.entry((a[i], a[i + 1])).or_default() += 1;
        *cnt_bc.entry((a[i + 1], a[i + 2])).or_default() += 1;
        *cnt_ac.entry((a[i], a[i + 2])).or_default() += 1;
    }
    let mut result: i64 = 0;
    for cnt in cnt_ab.into_values() {
        result += cnt as i64 * (cnt as i64 - 1) / 2;
    }
    for cnt in cnt_bc.into_values() {
        result += cnt as i64 * (cnt as i64 - 1) / 2;
    }
    for cnt in cnt_ac.into_values() {
        result += cnt as i64 * (cnt as i64 - 1) / 2;
    }
    for cnt in cnt_abc.into_values() {
        result -= 3 * cnt as i64 * (cnt as i64 - 1) / 2;
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
