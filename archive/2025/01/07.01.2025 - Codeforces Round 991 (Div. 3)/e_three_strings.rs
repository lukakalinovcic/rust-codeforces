//{"name":"E. Three Strings","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/E","interactive":false,"timeLimit":2500,"tests":[{"input":"7\na\nb\ncb\nab\ncd\nacbd\nab\nba\naabb\nxxx\nyyy\nxyxyxy\na\nbcd\ndecf\ncodes\nhorse\ncodeforces\negg\nannie\negaegaeg\n","output":"1\n0\n2\n0\n3\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EThreeStrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_line().into_bytes();
    let b = input.read_line().into_bytes();
    let c = input.read_line().into_bytes();
    let mut curr = vec![INF; b.len() + 1];
    curr[0] = 0;
    for i in 0..=a.len() {
        let mut next = vec![INF; b.len() + 1];
        for j in 0..=b.len() {
            if i < a.len() {
                next[j] = next[j].min(curr[j] + if c[i + j] == a[i] { 0 } else { 1 });
            }
            if j < b.len() {
                curr[j + 1] = curr[j + 1].min(curr[j] + if c[i + j] == b[j] { 0 } else { 1 });
            }
        }
        if i < a.len() {
            curr = next;
        }
    }
    out.print_line(curr[b.len()]);
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
