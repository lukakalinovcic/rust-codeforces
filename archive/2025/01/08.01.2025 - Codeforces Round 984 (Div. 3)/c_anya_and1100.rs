//{"name":"C. Anya and 1100","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n100\n4\n1 1\n2 0\n2 0\n3 1\n1100000\n3\n6 1\n7 1\n4 1\n111010\n4\n1 1\n5 0\n4 1\n5 0\n0100\n4\n3 1\n1 1\n2 0\n2 1\n","output":"NO\nNO\nNO\nNO\nYES\nYES\nNO\nNO\nYES\nYES\nYES\nNO\nNO\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAnyaAnd1100"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn check(i: i32, s: &Vec<u8>, active: &mut HashSet<usize>) {
    if i < 0 || i + 3 >= s.len() as i32 {
        return;
    }
    let i = i as usize;
    if s[i] == b'1' && s[i + 1] == b'1' && s[i + 2] == b'0' && s[i + 3] == b'0' {
        active.insert(i);
    } else {
        active.remove(&i);
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut s = input.read_line().into_bytes();
    let n = s.len();
    let mut active = HashSet::new();
    for i in 0..n - 3 {
        check(i as i32, &s, &mut active);
    }
    let q = input.read();
    for (p, v) in input.read_int_pair_vec(q) {
        let (p, v) = (p - 1, v as u8 + b'0');
        s[p as usize] = v;

        for i in (p - 3)..=p {
            check(i as i32, &s, &mut active);
        }

        if active.is_empty() {
            out.print_line("NO");
        } else {
            out.print_line("YES");
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
