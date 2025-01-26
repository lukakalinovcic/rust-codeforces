//{"name":"G. XOUR","group":"Codeforces - Codeforces Round 944 (Div. 4)","url":"https://codeforces.com/contest/1971/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n1 0 3 2\n5\n2 7 1 5 6\n8\n1 2 1 2 1 2 1 2\n4\n16 4 1 64\n","output":"0 1 2 3\n1 5 2 6 7\n1 1 1 1 2 2 2 2\n16 4 1 64\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GXOUR"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);
    let mut to_sort: HashMap<i32, Vec<_>> = HashMap::new();
    for i in 0..n {
        to_sort
            .entry(a[i] - (a[i] & 3))
            .or_default()
            .push((a[i], i));
    }
    for b in to_sort.into_values() {
        let (mut x, p): (Vec<_>, Vec<_>) = b.into_iter().unzip();
        x.sort();
        for (p, x) in p.into_iter().zip(x.into_iter()) {
            a[p] = x;
        }
    }
    out.print_iter(a.into_iter());
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
