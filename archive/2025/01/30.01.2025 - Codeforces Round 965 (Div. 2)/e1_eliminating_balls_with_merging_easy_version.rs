//{"name":"E1. Eliminating Balls With Merging (Easy Version)","group":"Codeforces - Codeforces Round 965 (Div. 2)","url":"https://codeforces.com/contest/1998/problem/E1","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n5 5\n1 2 3 2 1\n7 7\n4 5 1 2 1 4 5\n11 11\n1 2 3 1 1 9 3 2 4 1 3\n","output":"3\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1EliminatingBallsWithMergingEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _x = input.read_size();
    let a = input.read_long_vec(n);
    let mut lt = vec![n; n];
    let mut rt = vec![n; n];
    let mut s: Vec<(i64, usize)> = vec![];
    for i in 0..n {
        while !s.is_empty() && s.last().unwrap().0 <= a[i] {
            let (_, j) = s.pop().unwrap();
            lt[i] = j;
        }
        if !s.is_empty() {
            rt[s.last().unwrap().1] = i;
        }
        s.push((a[i], i));
    }

    out.print_line(rec(s[0].1, 0, &a, &lt, &rt).1);
}

fn rec(i: usize, threshold: i64, a: &[i64], lt: &[usize], rt: &[usize]) -> (i64, i32) {
    let mut sum = a[i];
    let mut cnt = 1;
    if lt[i] != a.len() {
        let (s, c) = rec(lt[i], a[i], a, lt, rt);
        sum += s;
        cnt += c;
    }
    if rt[i] != a.len() {
        let (s, c) = rec(rt[i], a[i], a, lt, rt);
        sum += s;
        cnt += c;
    }
    if sum < threshold {
        cnt = 0;
    }
    (sum, cnt)
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
