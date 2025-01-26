//{"name":"E. Find the Car","group":"Codeforces - Codeforces Round 944 (Div. 4)","url":"https://codeforces.com/contest/1971/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n10 1 3\n10\n10\n0\n6\n7\n10 2 4\n4 10\n4 7\n6\n4\n2\n7\n1000000000 1 1\n1000000000\n1000000000\n99999999\n6 1 3\n6\n5\n2\n6\n5\n","output":"0 6 7\n5 4 2 5\n99999999\n1 5 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFindTheCar"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_int();
    let k = input.read();
    let q = input.read();
    let a = input.read_int_vec(k);
    let b = input.read_int_vec(k);
    out.print_iter(input.read_int_vec(q).into_iter().map(|d| {
        let (Ok(i) | Err(i)) = a.binary_search(&d);
        let a0 = if i == 0 { 0 } else { a[i - 1] };
        let b0 = if i == 0 { 0 } else { b[i - 1] };
        b0 as i64 + (d - a0) as i64 * (b[i] - b0) as i64 / (a[i] - a0) as i64
    }));
    out.print_line("");
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
