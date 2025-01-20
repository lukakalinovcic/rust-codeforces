//{"name":"C. Alice's Adventures in Cutting Cake","group":"Codeforces - Codeforces Round 986 (Div. 2)","url":"https://codeforces.com/contest/2028/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n6 2 1\n1 1 10 1 1 10\n6 2 2\n1 1 10 1 1 10\n6 2 3\n1 1 10 1 1 10\n6 2 10\n1 1 10 1 1 10\n6 2 11\n1 1 10 1 1 10\n6 2 12\n1 1 10 1 1 10\n6 2 12\n1 1 1 1 10 10\n","output":"22\n12\n2\n2\n2\n0\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAlicesAdventuresInCuttingCake"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let v = input.read_long();
    let a = input.read_long_vec(n);
    let mut prefix_sum_for_k = vec![];
    let mut prefix_sum = 0;
    let mut sum = 0;
    let mut k = 0;
    prefix_sum_for_k.push(0);
    for ai in a.iter() {
        let ai = *ai;
        sum += ai;
        prefix_sum += ai;
        if sum >= v {
            k += 1;
            if k <= m {
                prefix_sum_for_k.push(prefix_sum);
            }
            sum = 0;
        }
    }
    let mut result = -1;
    if let Some(ps) = prefix_sum_for_k.get(m) {
        result = prefix_sum - ps;
    }
    k = 0;
    sum = 0;
    for ai in a.iter().rev() {
        let ai = *ai;
        sum += ai;
        prefix_sum -= ai;
        if sum >= v {
            k += 1;
            if k <= m {
                if let Some(ps) = prefix_sum_for_k.get(m - k) {
                    result = result.max(prefix_sum - ps);
                }
            }
            sum = 0;
        }
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
