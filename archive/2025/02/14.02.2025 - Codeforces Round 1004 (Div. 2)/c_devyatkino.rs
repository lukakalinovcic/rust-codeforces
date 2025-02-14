//{"name":"C. Devyatkino","group":"Codeforces - Codeforces Round 1004 (Div. 2)","url":"https://codeforces.com/contest/2067/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"16\n51\n60\n61\n777\n12345689\n1000000000\n2002\n3001\n977\n989898986\n80\n800001\n96\n70\n15\n90\n","output":"3\n2\n1\n0\n1\n3\n5\n4\n0\n7\n1\n2\n7\n0\n7\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDevyatkino"}}}

use std::collections::HashSet;
use std::ops::Div;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let mut seen = HashSet::new();
    let mut q = vec![(n, 0, None)];
    let mut i = 0;
    loop {
        let (x, d, prev) = q[i];
        i += 1;
        if has_seven(x) {
            out.print_line(d);
            return;
        }
        let mut p10 = 1;
        for j in 0..=10 {
            p10 *= 10;
            let y = x + p10 - 1;
            let explore = match prev {
                None => true,
                Some(prev) => j == prev,
            };
            if explore && seen.insert(y) {
                q.push((y, d + 1, Some(j)));
            }
        }
    }
}

fn has_seven(mut x: i64) -> bool {
    while x > 0 {
        let d = x % 10;
        if d == 7 {
            return true;
        }
        x /= 10;
    }
    false
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
