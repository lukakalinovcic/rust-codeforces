//{"name":"F. Bomb","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4\n5 6 7\n2 3 4\n5 9\n32 52 68 64 14\n18 14 53 24 8\n5 1000\n1 2 3 4 5\n5 4 3 2 1\n1 1000000\n1000000\n1\n10 6\n3 3 5 10 6 8 6 8 7 7\n6 1 7 4 1 1 8 9 3 1\n","output":"21\n349\n27\n500000500000\n47\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBomb"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn play_down_to(mut a: Vec<i32>, b: &Vec<i32>, lo: i32) -> (i64, i64, Vec<i32>) {
    let mut taken = 0;
    let mut score = 0;
    for (a, b) in a.iter_mut().zip(b.iter()) {
        let k = ((*a - lo + b - 1) / b).max(0);
        taken += k as i64;
        score += (*a + *a - (k - 1) * b) as i64 * k as i64 / 2;
        *a = (*a - k * b).max(0);
    }
    (taken, score, a)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_long();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut lo = 0;
    let mut hi = 1000000000;
    while lo < hi {
        let mid = (lo + hi) / 2;
        let (taken, _, _) = play_down_to(a.clone(), &b, mid);
        if taken <= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let (taken, mut score, mut a) = play_down_to(a.clone(), &b, lo);
    if lo > 0 {
        let k = k - taken;
        a.sort();
        a.reverse();
        for i in 0..k {
            score += a[i as usize] as i64;
        }
    }
    out.print_line(score);
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
