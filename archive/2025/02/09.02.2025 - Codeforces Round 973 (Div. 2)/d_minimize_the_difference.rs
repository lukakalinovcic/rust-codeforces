//{"name":"D. Minimize the Difference","group":"Codeforces - Codeforces Round 973 (Div. 2)","url":"https://codeforces.com/contest/2013/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1\n1\n3\n1 2 3\n4\n4 1 2 3\n4\n4 2 3 1\n5\n5 14 4 10 2\n","output":"0\n2\n1\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMinimizeTheDifference"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn first_last((total, len): (i64, i64)) -> (i64, i64) {
    let lo = total / len;
    let hi = if total % len == 0 { lo } else { lo + 1 };
    (hi, lo)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let mut s = vec![];
    for i in 0..n {
        let mut total = a[i];
        let mut len = 1;
        while !s.is_empty()
            && first_last(s.last().cloned().unwrap()).1 >= first_last((total, len)).0
        {
            let (t, l) = s.pop().unwrap();
            total += t;
            len += l;
        }
        s.push((total, len));
    }
    const INF: i64 = 1000000000000000000;
    let mut mini = INF;
    let mut maxi = 0;
    for (total, len) in s {
        let (hi, lo) = first_last((total, len));
        mini = mini.min(lo);
        maxi = maxi.max(hi);
    }
    out.print_line(maxi - mini);
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
