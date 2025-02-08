//{"name":"E. Computing Machine","group":"Codeforces - Codeforces Round 953 (Div. 2)","url":"https://codeforces.com/contest/1978/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n1111\n0000\n2\n1 2\n2 4\n4\n1010\n1101\n2\n1 3\n1 4\n6\n010101\n011010\n5\n2 3\n1 6\n2 5\n4 4\n3 6\n","output":"2\n3\n2\n3\n1\n4\n3\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EComputingMachine"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_line().into_bytes();
    let b = input.read_line().into_bytes();

    let mut b_range = vec![];
    for i in 0..n {
        if b[i] == b'1' {
            b_range.push(Some((i, i)));
        } else {
            let prev = if i == 0 { b'1' } else { a[i - 1] };
            let next = if i + 1 == n { b'1' } else { a[i + 1] };
            b_range.push(match (prev, next) {
                (b'0', b'0') => Some((i - 1, i + 1)),
                _ => None,
            });
        }
    }
    let mut a_range = vec![];
    for i in 0..n {
        if a[i] == b'1' {
            a_range.push(Some((i, i)));
        } else {
            let prev = if i == 0 { None } else { b_range[i - 1] };
            let next = if i + 1 == n { None } else { b_range[i + 1] };
            a_range.push(match (prev, next) {
                (Some((lo, _)), Some((_, hi))) => Some((lo, hi)),
                _ => None,
            });
        }
    }

    let mut prefix_some_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_some_sum[i + 1] = prefix_some_sum[i] + if a_range[i].is_some() { 1 } else { 0 };
    }

    let q = input.read_size();
    for (l, r) in input.read_int_pair_vec(q) {
        let l = l as usize - 1;
        let r = r as usize - 1;

        let mut result = 0;
        if r - l >= 4 {
            result += prefix_some_sum[r - 1] - prefix_some_sum[l + 2];
            for i in [l, l + 1, r - 1, r] {
                if let Some((lo, hi)) = a_range[i] {
                    if lo >= l && hi <= r {
                        result += 1;
                    }
                }
            }
        } else {
            for i in l..=r {
                if let Some((lo, hi)) = a_range[i] {
                    if lo >= l && hi <= r {
                        result += 1;
                    }
                }
            }
        }
        out.print_line(result);
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
