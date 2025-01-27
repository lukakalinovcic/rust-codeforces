//{"name":"C. Cirno and Operations","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/C#","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1\n-1000\n2\n5 -3\n2\n1000 1\n9\n9 7 9 -9 9 -8 7 -8 9\n11\n678 201 340 444 453 922 128 987 127 752 0\n","output":"-1000\n8\n1001\n2056\n269891\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCirnoAndOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    out.print_line(doit(a));
}

fn doit(mut a: Vec<i64>) -> i64 {
    let mut result = do_sum(&a);
    while a.len() > 1 {
        let mut b = vec![];
        for i in 1..a.len() {
            b.push(a[i] - a[i - 1]);
        }
        let sum = do_sum(&b);
        result = result.max(sum.max(-sum));
        a = b;
    }
    result
}

fn do_sum(a: &[i64]) -> i64 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i];
    }
    sum
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
