//{"name":"D. Game With Triangles","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 3\n0\n0 1 -1\n2 4\n0 100\n-100 -50 0 50\n2 4\n0 1000\n-100 -50 0 50\n6 6\n20 1 27 100 43 42\n100 84 1 24 22 77\n8 2\n564040265 -509489796 469913620 198872582 -400714529 553177666 131159391 -20796763\n-1000000000 1000000000\n","output":"1\n2\n2\n150 200\n2\n1000 200\n4\n99 198 260 283\n2\n2000000000 2027422256\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGameWithTriangles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut a = input.read_int_vec(n);
    let mut b = input.read_int_vec(m);
    a.sort();
    b.sort();
    let a_k = |k: usize| (a[n - 1 - k] - a[k]) as i64;
    let b_k = |k: usize| (b[m - 1 - k] - b[k]) as i64;
    let mut result = 0;
    let mut n_a = 0;
    let mut n_b = 0;
    let mut results = vec![];
    loop {
        let rem_a = n - 2 * n_a - n_b;
        let rem_b = m - 2 * n_b - n_a;
        let add_a = if rem_a >= 2 && rem_b >= 1 {
            Some(a_k(n_a))
        } else {
            None
        };
        let add_b = if rem_b >= 2 && rem_a >= 1 {
            Some(b_k(n_b))
        } else {
            None
        };
        match (add_a, add_b) {
            (Some(add_a), Some(add_b)) => {
                if add_a > add_b {
                    result += add_a;
                    n_a += 1;
                } else {
                    result += add_b;
                    n_b += 1;
                }
            }
            (Some(add_a), None) => {
                result += add_a;
                n_a += 1;
            }
            (None, Some(add_b)) => {
                result += add_b;
                n_b += 1;
            }
            (None, None) => {
                if rem_a >= 3 && n_b > 0 {
                    n_b -= 1;
                    result -= b_k(n_b);
                    result += a_k(n_a);
                    n_a += 1;
                    result += a_k(n_a);
                    n_a += 1;
                } else if rem_b >= 3 && n_a > 0 {
                    n_a -= 1;
                    result -= a_k(n_a);
                    result += b_k(n_b);
                    n_b += 1;
                    result += b_k(n_b);
                    n_b += 1;
                } else {
                    break;
                }
            }
        }
        results.push(result);
    }
    out.print_line(results.len());
    out.print_iter(results.into_iter());
    out.print_line("");
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
