//{"name":"E. Prefix GCD","group":"Codeforces - Codeforces Round 973 (Div. 2)","url":"https://codeforces.com/contest/2013/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n4 2 2\n2\n6 3\n3\n10 15 6\n5\n6 42 12 52 20\n4\n42 154 231 66\n","output":"6\n6\n9\n14\n51\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPrefixGCD"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);
    let mut g1 = 0;
    for i in 0..n {
        g1 = gcd(g1, a[i]);
    }
    for i in 0..n {
        a[i] /= g1;
    }
    let mut sum_g2 = 0;
    let mut g2 = 0;
    for j in 0..n {
        let mut mini = -1;
        for i in 0..n {
            if a[i] == -1 {
                continue;
            }
            if mini == -1 || gcd(g2, a[i]) < gcd(g2, a[mini as usize]) {
                mini = i as i32;
            }
        }
        g2 = gcd(g2, a[mini as usize]);
        a[mini as usize] = -1;
        if g2 == 1 {
            sum_g2 += (n - j) as i64;
            break;
        }
        sum_g2 += g2 as i64;
    }
    out.print_line(g1 as i64 * sum_g2 as i64);
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
