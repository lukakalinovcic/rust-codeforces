//{"name":"D. Elections","group":"Codeforces - Codeforces Round 953 (Div. 2)","url":"https://codeforces.com/contest/1978/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 1\n2 0 3\n2 3\n0 10\n5 3\n5 4 3 2 1\n4 5\n3 10 7 1\n6 0\n2 2 2 3 3 3\n","output":"0 1 2\n1 0\n0 1 2 3 4\n1 0 2 3\n1 1 2 0 4 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DElections"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let c = input.read_int();
    let mut a = input.read_int_vec(n);
    a[0] += c;
    let mut suffix_max = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_max[i] = a[i].max(suffix_max[i + 1]);
    }
    let mut prefix_max = 0;
    let mut prefix_sum: i64 = 0;
    let mut result = vec![];
    for i in 0..n {
        if a[i] >= suffix_max[i] && a[i] > prefix_max {
            result.push(0);
        } else if prefix_sum + a[i] as i64 >= suffix_max[i + 1] as i64 {
            result.push(i);
        } else {
            result.push(i + 1);
        }
        prefix_max = prefix_max.max(a[i]);
        prefix_sum += a[i] as i64;
    }
    out.print_line(result);
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
