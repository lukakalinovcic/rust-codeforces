//{"name":"C. Cards Partition","group":"Codeforces - Codeforces Round 975 (Div. 2)","url":"https://codeforces.com/contest/2019/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n3 1\n3 2 2\n5 4\n2 6 1 2 4\n2 100\n1410065408 10000000000\n10 8\n7 4 6 6 9 3 10 2 8 7\n2 12\n2 2\n2 70\n0 1\n1 0\n1\n3 0\n2 1 2\n3 1\n0 3 3\n","output":"2\n3\n1\n7\n2\n2\n1\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCardsPartition"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_long();
    let mut a = input.read_long_vec(n);
    a.sort();
    a.reverse();
    let mut sum_a = 0;
    for i in 0..n {
        sum_a += a[i];
    }
    let mut result = 0;
    for deck_size in 1..=n {
        let extra = (sum_a + k) % deck_size as i64;
        let total = sum_a + k - extra;
        if total < sum_a {
            continue;
        }
        let decks = total / deck_size as i64;
        if a[0] > decks {
            continue;
        }
        result = result.max(deck_size);
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
