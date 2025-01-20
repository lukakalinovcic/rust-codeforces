//{"name":"B. Transfusion","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3\n3 2 1\n3\n1 1 3\n4\n1 2 5 4\n4\n1 6 6 1\n5\n6 2 1 4 2\n4\n1 4 2 1\n5\n3 1 2 1 3\n3\n2 4 2\n","output":"YES\nNO\nYES\nNO\nYES\nNO\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTransfusion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let a = input.read_int_vec(n);
    let mut even_sum = 0;
    let mut odd_sum = 0;
    for i in 0..n {
        if i % 2 == 0 {
            even_sum += a[i] as i64;
        } else {
            odd_sum += a[i] as i64;
        }
    }
    let even_cnt = ((n + 1) / 2) as i64;
    let odd_cnt = (n / 2) as i64;
    if even_sum % even_cnt != 0
        || odd_sum % odd_cnt != 0
        || even_sum / even_cnt != odd_sum / odd_cnt
    {
        out.print_line("NO");
    } else {
        out.print_line("YES");
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
