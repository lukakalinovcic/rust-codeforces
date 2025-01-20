//{"name":"C. Sofia and the Lost Operations","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3\n1 2 1\n1 3 2\n4\n1 3 1 2\n4\n1 2 3 5\n2 1 3 5\n2\n2 3\n5\n7 6 1 10 10\n3 6 1 11 11\n3\n4 3 11\n4\n3 1 7 8\n2 2 7 10\n5\n10 3 2 2 1\n5\n5 7 1 7 9\n4 10 1 2 9\n8\n1 1 9 8 7 2 10 4\n4\n1000000000 203 203 203\n203 1000000000 203 1000000000\n2\n203 1000000000\n1\n1\n1\n5\n1 3 4 5 1\n","output":"YES\nNO\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSofiaAndTheLostOperations"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut need = HashMap::new();
    for i in 0..n {
        let cnt = need.entry(&b[i]).or_insert(0);
        if a[i] != b[i] {
            *cnt += 1;
        }
    }
    let m = input.read();
    for (i, x) in input.read_int_vec(m).into_iter().enumerate() {
        match need.get_mut(&x) {
            None => {
                if i == m - 1 {
                    out.print_line("NO");
                    return;
                }
            }
            Some(cnt) => *cnt -= 1,
        }
    }
    for cnt in need.values() {
        if *cnt > 0 {
            out.print_line("NO");
            return;
        }
    }
    out.print_line("YES");
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
