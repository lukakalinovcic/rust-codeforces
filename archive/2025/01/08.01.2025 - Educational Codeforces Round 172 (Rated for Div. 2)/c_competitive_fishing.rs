//{"name":"C. Competitive Fishing","group":"Codeforces - Educational Codeforces Round 172 (Rated for Div. 2)","url":"https://codeforces.com/contest/2042/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4 1\n1001\n4 1\n1010\n4 1\n0110\n4 2\n0110\n6 3\n001110\n10 20\n1111111111\n5 11\n11111\n","output":"2\n-1\n2\n-1\n3\n4\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCompetitiveFishing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n: usize = input.read();
    let mut k = input.read_int();
    let s = input.read_line().into_bytes();
    let mut diffs = vec![];
    let mut diff = 0;
    for ch in s.into_iter().rev() {
        diff += if ch == b'1' { 1 } else { -1 };
        diffs.push(diff);
    }
    diffs.remove(diffs.len() - 1);
    diffs.sort();
    diffs.reverse();
    eprintln!("{diffs:?}");
    for (i, d) in diffs.into_iter().enumerate() {
        if d >= 0 {
            k -= d;
        }
        if k <= 0 {
            out.print_line(i + 2);
            return;
        }
    }
    out.print_line("-1");
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
