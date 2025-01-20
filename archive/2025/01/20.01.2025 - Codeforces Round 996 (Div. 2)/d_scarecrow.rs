//{"name":"D. Scarecrow","group":"Codeforces - Codeforces Round 996 (Div. 2)","url":"https://codeforces.com/contest/2055/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n1 3 5\n0\n3 2 5\n2 5 5\n1 10 10\n10\n10 1 10\n0 1 2 3 4 5 6 7 8 9\n2 1 2\n0 0\n2 1 2\n0 2\n2 1 3\n0 2\n2 2 4\n1 1\n9 12 54\n3 3 8 24 25 27 29 34 53\n","output":"4\n5\n20\n0\n2\n1\n2\n2\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DScarecrow"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_int() * 2;
    let l = input.read_int() * 2;
    let a = input.read_int_vec(n);
    let mut p = vec![];
    for a in a {
        let a = 2 * a;
        while !p.is_empty() {
            let (t, x) = p.last().cloned().unwrap();
            let x_min: i32 = a - t;
            let x_max: i32 = a + t;
            if x_min > x {
                let dt = (x_min - x) / 2;
                let t2 = t + dt;
                let x2 = x_min - dt;
                p.push((t2, x2 + k));
                break;
            } else if x_max >= x - k {
                p.last_mut().unwrap().1 = x_max.min(x) + k;
                break;
            } else {
                p.pop().unwrap();
            }
        }
        if p.is_empty() {
            p.push((a, k));
        }
    }
    let mut result = 1000000000;
    while !p.is_empty() {
        let (t, x) = p.pop().unwrap();
        if x >= l {
            result = result.min(t);
        } else {
            result = result.min(t + (l - x));
            break;
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
