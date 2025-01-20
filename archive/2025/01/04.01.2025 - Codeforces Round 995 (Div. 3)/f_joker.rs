//{"name":"F. Joker","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 5 3\n1 2 3\n2 1 4\n2 1 1 2\n5 3 1\n3\n3 2 4\n2 1 1 1\n18 15 4\n13 15 1 16\n","output":"2 3 5\n2 2 2 2\n2\n2 3 3 3\n2 4 6 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FJoker"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, m, q) = (input.read_int(), input.read_int(), input.read_int());
    let mut ranges = [(m, m)].to_vec();
    let result = (0..q).map(|_| {
        let a = input.read_int();
        let mut hit = false;
        ranges = ranges
            .clone()
            .into_iter()
            .map(|(lo, hi)| {
                if a < lo {
                    Some((lo - 1, hi))
                } else if a > hi {
                    Some((lo, hi + 1))
                } else {
                    hit = true;
                    if lo == hi {
                        None
                    } else {
                        Some((lo, hi))
                    }
                }
            })
            .flatten()
            .collect();
        if hit {
            ranges.insert(0, (1, 1));
            ranges.push((n, n));
        }

        let mut i = 0;
        let mut total = 0;
        while i < ranges.len() {
            if i + 1 < ranges.len() && ranges[i].1 >= ranges[i + 1].0 {
                ranges[i].1 = ranges[i + 1].1;
                ranges.remove(i + 1);
            } else {
                total += ranges[i].1 - ranges[i].0 + 1;
                i += 1;
            }
        }
        total
    });
    out.print_iter(result);
    out.put(b'\n');
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
