//{"name":"B. Arranging Cats","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/contest/1921/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\n10010\n00001\n1\n1\n1\n3\n000\n111\n4\n0101\n1010\n3\n100\n101\n8\n10011001\n11111110\n","output":"2\n0\n3\n2\n1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BArrangingCats"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_line().into_bytes();
    let b = input.read_line().into_bytes();
    let mut a_ones = 0;
    let mut b_ones = 0;
    for i in 0..n {
        if a[i] != b[i] {
            if a[i] == b'1' {
                a_ones += 1;
            } else if b[i] == b'1' {
                b_ones += 1;
            }
        }
    }
    out.print_line(a_ones.max(b_ones));
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
