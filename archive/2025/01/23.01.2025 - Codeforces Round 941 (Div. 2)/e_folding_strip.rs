//{"name":"E. Folding Strip","group":"Codeforces - Codeforces Round 941 (Div. 2)","url":"https://codeforces.com/contest/1966/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6\n101101\n1\n0\n12\n110110110011\n5\n01110\n4\n1111\n2\n01\n","output":"3\n1\n3\n3\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFoldingStrip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut s = input.read_line().into_bytes();
    let r1 = doit(n, &s);
    s.reverse();
    let r2 = doit(n, &s);
    out.print_line(r1.min(r2));
}

fn doit(n: usize, s: &[u8]) -> i32 {
    let mut mini = 0;
    let mut maxi = 0;
    let mut dir = 1;
    let mut pos = 0;
    for i in 1..=n {
        pos += dir;
        mini = mini.min(pos);
        maxi = maxi.max(pos);
        if i != n && s[i] == s[i - 1] {
            dir = -dir;
        }
    }
    maxi - mini
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
