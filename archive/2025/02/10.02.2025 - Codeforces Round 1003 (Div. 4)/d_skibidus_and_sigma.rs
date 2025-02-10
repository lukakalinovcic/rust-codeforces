//{"name":"D. Skibidus and Sigma","group":"Codeforces - Codeforces Round 1003 (Div. 4)","url":"https://codeforces.com/contest/2065/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2\n4 4\n6 1\n3 4\n2 2 2 2\n3 2 1 2\n4 1 2 1\n2 3\n3 4 5\n1 1 9\n","output":"41\n162\n72\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSkibidusAndSigma"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut t = vec![];
    for _ in 0..n {
        let a = input.read_int_vec(m);
        let mut s: i64 = 0;
        let mut ss: i64 = 0;
        for j in 0..m {
            s += a[j] as i64;
            ss += s;
        }
        t.push((s, ss));
    }
    t.sort();
    t.reverse();
    let mut s: i64 = 0;
    let mut ss: i64 = 0;
    for (t, tt) in t {
        ss += tt + m as i64 * s;
        s += t;
    }
    out.print_line(ss);
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
