//{"name":"A. Game of Division","group":"Codeforces - Codeforces Round 992 (Div. 2)","url":"https://codeforces.com/contest/2040/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n3 2\n1 2 3\n4 2\n1 2 4 5\n5 3\n10 7 3 4 5\n5 3\n1 31 15 55 36\n2 1\n17 17\n2 2\n17 18\n1 3\n6\n","output":"YES\n2\nNO\nYES\n3\nNO\nNO\nYES\n2\nYES\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGameOfDivision"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let k = input.read_int();
    let a = input.read_int_vec(n);
    match a.iter().enumerate().find(|(i, x)| {
        let (i, x) = (*i, *x);
        a.iter()
            .enumerate()
            .all(|(j, y)| (x - y).abs() % k != 0 || i == j)
    }) {
        Some((i, _)) => {
            out.print_line("YES");
            out.print_line(i + 1);
        }
        None => out.print_line("NO"),
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
