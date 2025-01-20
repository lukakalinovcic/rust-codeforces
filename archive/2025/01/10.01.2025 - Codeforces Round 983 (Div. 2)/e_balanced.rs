//{"name":"E. Balanced","group":"Codeforces - Codeforces Round 983 (Div. 2)","url":"https://codeforces.com/contest/2032/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3\n2 1 2\n3\n1 2 3\n5\n1 2 1 2 1\n7\n1 2 1 2 1 3 1\n9\n10000 10000 10000 10000 10000 10001 10002 10001 10000\n1\n10\n","output":"0 1 0\n2 1 0\n2 0 3 0 2\n4 2 7 0 8 0 6\n1 1 1 1 1 1 0 1 1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBalanced"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    if n == 1 {
        out.print_line(0);
        return;
    }
    let mut sum = 0;
    for x in a.iter() {
        sum += *x;
    }
    let mut k = 0;
    while (sum + k * 4) % (n as i64) != 0 {
        k += 1;
    }
    let target = (sum + k * 4) / (n as i64);

    let (mut x0, mut x1, mut xn) = (1, 0, target - a[n - 2]);
    let (mut y0, mut y1, mut yn) = (2, 1, target - a[n - 1]);
    for i in 0..n - 2 {
        if i == n - 3 {
            x1 += 1;
        }
        (x0, x1, xn) = (x1 - x0 * 2, -x0, xn - x0 * (target - a[i]));
        (y0, y1, yn) = (y1 - y0 * 2, -y0, yn - y0 * (target - a[i]));
    }
    x1 += 2;
    y1 += 1;
    let det = x0 * y1 - x1 * y0;
    let det0 = xn * y1 - yn * x1;
    let det1 = yn * x0 - xn * y0;
    let mut v = vec![0; n];
    v[n - 3] = det0 / det;
    v[n - 2] = det1 / det;
    for i in (0..n - 2).rev() {
        v[(i + n - 1) % n] = target - a[i] - v[i + 1] - 2 * v[i];
    }
    let mut mini = v[0];
    for v in v.iter() {
        mini = mini.min(*v);
    }
    for i in 0..n {
        v[i] -= mini;
    }
    out.print_iter(v.into_iter());
    out.print("\n");
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
