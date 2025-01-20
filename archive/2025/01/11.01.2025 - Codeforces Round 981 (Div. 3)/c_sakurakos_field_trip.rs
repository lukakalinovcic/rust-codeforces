//{"name":"C. Sakurako's Field Trip","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n5\n1 1 1 2 3\n6\n2 1 2 2 1 1\n4\n1 2 1 1\n6\n2 1 1 2 2 4\n4\n2 1 2 3\n6\n1 2 2 1 2 1\n5\n4 5 5 1 5\n7\n1 4 3 5 1 1 3\n7\n3 1 3 2 2 3 3\n","output":"1\n2\n1\n0\n0\n1\n1\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSakurakosFieldTrip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn cost(a: i32, b: i32) -> i32 {
    if a == b {
        1
    } else {
        0
    }
}

fn pair_cost(a: (i32, i32), b: (i32, i32)) -> i32 {
    cost(a.0, b.0) + cost(a.1, b.1)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut prev_normal = 0;
    let mut prev_swapped = 0;
    for i in 1..n / 2 {
        let p_normal = (a[i - 1], a[n - i]);
        let p_swapped = (a[n - i], a[i - 1]);
        let c_normal = (a[i], a[n - i - 1]);
        let c_swapped = (a[n - i - 1], a[i]);

        let curr_normal = (prev_swapped + pair_cost(p_swapped, c_normal))
            .min(prev_normal + pair_cost(p_normal, c_normal));
        let curr_swapped = (prev_swapped + pair_cost(p_swapped, c_swapped))
            .min(prev_normal + pair_cost(p_normal, c_swapped));

        prev_normal = curr_normal;
        prev_swapped = curr_swapped;
    }
    let result = prev_normal.min(prev_swapped)
        + if n % 2 == 0 {
            cost(a[n / 2 - 1], a[n / 2])
        } else {
            cost(a[n / 2 - 1], a[n / 2]) + cost(a[n / 2], a[n / 2 + 1])
        };
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
