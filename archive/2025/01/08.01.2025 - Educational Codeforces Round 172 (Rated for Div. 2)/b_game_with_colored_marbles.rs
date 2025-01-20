//{"name":"B. Game with Colored Marbles","group":"Codeforces - Educational Codeforces Round 172 (Rated for Div. 2)","url":"https://codeforces.com/contest/2042/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n1 3 1 3 4\n3\n1 2 3\n4\n4 4 4 4\n","output":"4\n4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGameWithColoredMarbles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let c = input.read_int_vec(n);
    let mut cnt = vec![0; n + 1];
    for ci in c {
        cnt[ci as usize] += 1;
    }
    let mut ones = 0;
    let mut rest = 0;
    for x in cnt {
        if x == 1 {
            ones += 1;
        } else if x > 1 {
            rest += 1;
        }
    }
    out.print_line(rest + 2 * ((ones + 1) / 2));
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
