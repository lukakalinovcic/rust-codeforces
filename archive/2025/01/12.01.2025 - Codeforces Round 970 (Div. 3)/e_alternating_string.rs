//{"name":"E. Alternating String","group":"Codeforces - Codeforces Round 970 (Div. 3)","url":"https://codeforces.com/contest/2008/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n1\na\n2\nca\n3\naab\n5\nababa\n6\nacdada\n9\nejibmyyju\n6\nbbccbc\n6\nabacba\n5\nbcbca\n5\ndcbdb\n","output":"1\n0\n1\n1\n2\n6\n2\n3\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAlternatingString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let s = input.read_line().into_bytes();
    let mut prefix_skip_sum = vec![vec![0; 26]; n + 2];
    for i in 0..n {
        for c in 0..26 {
            prefix_skip_sum[i + 2][c] =
                prefix_skip_sum[i][c] + if s[i] == b'a' + c as u8 { 1 } else { 0 };
        }
    }
    let result = if n % 2 == 0 {
        let mut a = n;
        let mut b = n;
        for c in 0..26 {
            a = a.min((n + 1) / 2 - prefix_skip_sum[n + 1][c]);
            b = b.min(n / 2 - prefix_skip_sum[n][c]);
        }
        a + b
    } else {
        let mut result = 1 + n;
        let mut suffix_skip_sum = vec![vec![0; 26]; 2];
        for i in (0..n).rev() {
            let curr = i % 2;
            let next = (i + 1) % 2;
            let mut a = n;
            let mut b = n;
            for c in 0..26 {
                a = a.min((n) / 2 - prefix_skip_sum[i + 1][c] - suffix_skip_sum[curr][c]);
                b = b.min((n - 1) / 2 - prefix_skip_sum[i][c] - suffix_skip_sum[next][c]);
            }
            result = result.min(1 + a + b);
            for c in 0..26 {
                suffix_skip_sum[curr][c] += if s[i] == b'a' + c as u8 { 1 } else { 0 };
            }
        }
        result
    };
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
