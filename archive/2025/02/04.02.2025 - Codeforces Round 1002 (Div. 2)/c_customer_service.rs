//{"name":"C. Customer Service","group":"Codeforces - Codeforces Round 1002 (Div. 2)","url":"https://codeforces.com/contest/2059/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 2\n2 1\n2\n10 10\n10 10\n3\n2 3 3\n4 4 1\n2 1 1\n4\n4 2 2 17\n1 9 3 1\n5 5 5 11\n1 2 1 1\n","output":"2\n1\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCustomerService"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = (0..n).map(|_| input.read_int_vec(n)).collect::<Vec<_>>();
    let mut cnt = vec![0; n + 1];
    for i in 0..n {
        for j in (0..n).rev() {
            cnt[n - j - 1] += 1;
            if a[i][j] != 1 {
                break;
            }
        }
    }
    for result in 1.. {
        let mut ok = true;
        for i in 0..result {
            ok &= cnt[i] >= result - i;
        }
        if !ok {
            out.print_line(result - 1);
            break;
        }
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
