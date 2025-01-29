//{"name":"F. Eri and Expanded Sets","group":"Codeforces - Codeforces Round 969 (Div. 2)","url":"https://codeforces.com/contest/2007/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2\n2 2\n6\n1 3 6 10 15 21\n5\n6 30 18 36 9\n1\n1000000000\n6\n1 1 4 5 1 4\n12\n70 130 90 90 90 108 612 500 451 171 193 193\n","output":"3\n18\n5\n1\n18\n53\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEriAndExpandedSets"}}}

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
    let a = input.read_int_vec(n);

    let mut next = vec![n; n];
    for i in (0..n - 1).rev() {
        if a[i] == a[i + 1] {
            next[i] = next[i + 1];
        } else {
            next[i] = i + 1;
        }
    }

    let m = n.next_power_of_two().trailing_zeros() as usize;
    let mut p = vec![vec![0; n]; m];
    for i in 1..n {
        p[0][i] = (a[i] - a[i - 1]).abs();
    }
    for k in 1..m {
        let half = 1 << (k - 1);
        if n > half {
            for i in 1..(n - half) {
                p[k][i] = gcd(p[k - 1][i], p[k - 1][i + half]);
            }
        }
    }

    let mut result = n as i64 * (n as i64 + 1) / 2;
    for i in 0..n {
        let j = next[i];
        if j == n {
            continue;
        }
        let mut d = (a[j] - a[i]).abs();
        while d % 2 == 0 {
            d /= 2;
        }
        if d == 1 {
            continue;
        }
        let mut lo = j + 1;
        let mut hi = n;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let k = ((mid - j + 1).next_power_of_two().trailing_zeros() - 1) as usize;
            let g = gcd(d, gcd(p[k][j + 1], p[k][mid - (1 << k) + 1]));

            if g == 1 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        result -= (lo - j) as i64;
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
