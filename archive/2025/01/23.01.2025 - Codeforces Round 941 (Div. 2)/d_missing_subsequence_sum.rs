//{"name":"D. Missing Subsequence Sum","group":"Codeforces - Codeforces Round 941 (Div. 2)","url":"https://codeforces.com/contest/1966/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 2\n6 1\n8 8\n9 3\n10 7\n","output":"1\n1\n5\n2 3 4 5 6\n7\n1 1 1 1 1 1 1\n4\n7 1 4 1\n4\n1 2 8 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMissingSubsequenceSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let k = input.read_int();
    let mut a = vec![];
    let (lo, mut hi) = if k == 1 {
        a.push(2);
        a.push(3);
        a.push(4);
        (2, 7)
    } else {
        let mut p = 1;
        while 2 * p - 1 < k {
            a.push(p);
            p *= 2;
        }
        if p - 1 < k - 1 {
            a.push((k - 1) - (p - 1));
        }
        a.push(k + 1);
        a.push(2 * k);
        (k + 1, 3 * k - 1)
    };
    while hi < n {
        let x = hi + 1 - lo;
        a.push(x);
        hi = x + hi;
    }
    out.print_line(a.len());
    out.print_iter(a.into_iter());
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
