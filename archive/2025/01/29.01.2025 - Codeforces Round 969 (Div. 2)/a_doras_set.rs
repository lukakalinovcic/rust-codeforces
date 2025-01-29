//{"name":"A. Dora's Set","group":"Codeforces - Codeforces Round 969 (Div. 2)","url":"https://codeforces.com/contest/2007/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1 3\n3 7\n10 21\n2 8\n51 60\n2 15\n10 26\n1 1000\n","output":"1\n1\n3\n1\n2\n3\n4\n250\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADorasSet"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_int();
    let r = input.read_int();
    let mut taken = vec![false; 1001];
    let mut result = 0;
    for a in (l..=r).rev() {
        if taken[a as usize] {
            continue;
        }
        for b in a + 1..=r.min(a + 4) {
            if taken[b as usize] || gcd(a, b) != 1 {
                continue;
            }
            for c in b + 1..=r.min(b + 4) {
                if taken[c as usize] || gcd(a, c) != 1 || gcd(b, c) != 1 {
                    continue;
                }
                taken[a as usize] = true;
                taken[b as usize] = true;
                taken[c as usize] = true;
                result += 1;
            }
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
