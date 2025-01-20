//{"name":"C. Numeric String Template","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n3 5 2 1 3\n2\nabfda\nafbfa\n2\n1 2\n3\nab\nabc\naa\n4\n5 -3 5 -3\n4\naaaa\nbcbc\naba\ncbcb\n","output":"YES\nNO\nYES\nNO\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNumericStringTemplate"}}}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn canonical(mut a: Vec<i32>) -> Vec<i32> {
    let mut assigned = HashMap::new();
    let mut next_value = 0;
    for a in a.iter_mut() {
        *a = match assigned.entry(*a) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let e = e.insert(next_value);
                next_value += 1;
                *e
            }
        };
    }
    a
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let m = input.read();
    let a = canonical(a);
    for _ in 0..m {
        let b = input
            .read_line()
            .into_bytes()
            .into_iter()
            .map(|ch| (ch - b'a') as i32)
            .collect::<Vec<_>>();
        let b = canonical(b);
        if a == b {
            out.print_line("YES");
        } else {
            out.print_line("NO");
        }
    }
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
