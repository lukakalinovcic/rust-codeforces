//{"name":"D. Robert Hood and Mrs Hood","group":"Codeforces - Codeforces Round 974 (Div. 3)","url":"https://codeforces.com/contest/2014/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 1 1\n1 2\n4 1 2\n1 2\n2 4\n7 2 3\n1 2\n1 3\n6 7\n5 1 2\n1 2\n3 5\n9 2 1\n2 8\n9 2 4\n7 9\n4 8\n1 3\n2 3\n","output":"1 1\n2 1\n1 4\n1 1\n1 1\n3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRobertHoodAndMrsHood"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let d = input.read();
    let k = input.read();
    let mut starts = vec![0; n + 1];
    let mut ends = vec![0; n + 1];
    for (a, b) in input.read_int_pair_vec(k) {
        let (a, b) = (a as usize, b as usize);
        starts[a] += 1;
        ends[b] += 1;
    }
    let mut started = 0;
    let mut min = None;
    let mut max = None;
    for i in 1..=n {
        started += starts[i];
        ends[i] += ends[i - 1];
        if i < d {
            continue;
        }
        let k = started - ends[i - d];
        let p = i - d + 1;
        min = Some(match min {
            None => (k, p),
            Some((min, pos)) => {
                if k < min {
                    (k, p)
                } else {
                    (min, pos)
                }
            }
        });
        max = Some(match max {
            None => (k, p),
            Some((max, pos)) => {
                if k > max {
                    (k, p)
                } else {
                    (max, pos)
                }
            }
        });
    }
    out.print_line(format!("{} {}", max.unwrap().1, min.unwrap().1));
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
