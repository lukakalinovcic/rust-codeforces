//{"name":"B. Choosing Cubes","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"12\n5 2 2\n4 3 3 2 3\n5 5 3\n4 2 1 3 5\n5 5 2\n5 2 4 1 3\n5 5 5\n1 2 5 4 3\n5 5 4\n3 1 2 4 5\n5 5 5\n4 3 2 1 5\n6 5 3\n1 2 3 1 2 3\n10 1 1\n1 1 1 1 1 1 1 1 1 1\n1 1 1\n42\n5 2 3\n2 2 1 1 2\n2 1 1\n2 1\n5 3 1\n3 3 2 3 2\n","output":"MAYBE\nYES\nNO\nYES\nYES\nYES\nMAYBE\nMAYBE\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BChoosingCubes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let f = input.read::<usize>() - 1;
    let k = input.read_int();
    let a = input.read_int_vec(n);
    let mut cnt_gt = 0;
    let mut cnt_gte = 0;
    for i in 0..n {
        if i != f {
            if a[i] > a[f] {
                cnt_gt += 1;
            }
            if a[i] >= a[f] {
                cnt_gte += 1;
            }
        }
    }
    if cnt_gte < k {
        out.print_line("YES");
    } else if cnt_gt < k {
        out.print_line("MAYBE");
    } else {
        out.print_line("NO");
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
