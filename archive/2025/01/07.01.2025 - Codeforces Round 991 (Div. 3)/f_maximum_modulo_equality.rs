//{"name":"F. Maximum modulo equality","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n5 5\n5 14 2 6 3\n4 5\n1 4\n2 4\n3 5\n1 1\n1 1\n7\n1 1\n3 2\n1 7 8\n2 3\n1 2\n","output":"3 1 4 1 0\n0\n1 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMaximumModuloEquality"}}}

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

fn build_tree(tree: &mut [i32], i: usize, lo: usize, hi: usize, a: &[i32]) -> i32 {
    if hi == lo {
        return 0;
    }
    if hi - lo == 1 {
        tree[i] = (a[lo] - a[lo - 1]).abs();
    } else {
        let mid = (lo + hi) / 2;
        let x = build_tree(tree, 2 * i, lo, mid, a);
        let y = build_tree(tree, 2 * i + 1, mid, hi, a);
        tree[i] = gcd(x, y);
    }
    return tree[i];
}

fn query_tree(tree: &[i32], i: usize, lo: usize, hi: usize, l: usize, r: usize) -> i32 {
    if lo >= l && hi <= r {
        return tree[i];
    }
    if r <= lo || l >= hi {
        return 0;
    }
    let mid = (lo + hi) / 2;
    let x = query_tree(tree, 2 * i, lo, mid, l, r);
    let y = query_tree(tree, 2 * i + 1, mid, hi, l, r);
    return gcd(x, y);
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let q: usize = input.read();
    let a = input.read_int_vec(n);
    let mut m = 1;
    while m < 2 * a.len() {
        m *= 2;
    }
    let mut tree = vec![0; m];
    build_tree(&mut tree, 1, 1, n, &a);
    for _ in 0..q {
        let l: usize = input.read();
        let r: usize = input.read();
        out.print_line(query_tree(&tree, 1, 1, n, l, r));
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
