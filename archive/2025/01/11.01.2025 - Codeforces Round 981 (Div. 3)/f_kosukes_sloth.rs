//{"name":"F. Kosuke's Sloth","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2\n100 1\n1000000000000 1377\n","output":"9\n100\n999244007\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKosukesSloth"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i64 = 1000000007;

fn add(a: i64, b: i64) -> i64 {
    (a + b) % MOD
}

fn mult(a: i64, b: i64) -> i64 {
    ((a % MOD) * (b % MOD)) % MOD
}

fn find_cycle(k: i32) -> (i64, i64, Vec<i64>) {
    if k == 1 {
        return (1, 1, vec![0]);
    }
    let mut a = 1;
    let mut b = 1;
    let mut seen = HashMap::new();
    let mut zeros = vec![];
    for i in 3.. {
        let c = (a + b) % k;
        a = b;
        b = c;
        if let Some(cycle_start) = seen.insert((a, b), i) {
            let cycle_len = i as i32 - cycle_start;
            let mut p = 0;
            while p < zeros.len() && zeros[p] < cycle_start {
                p += 1;
            }
            return (
                cycle_start as i64,
                cycle_len as i64,
                zeros[p..]
                    .into_iter()
                    .map(|x| (x - cycle_start) as i64)
                    .collect(),
            );
        }
        if b == 0 {
            zeros.push(i);
        }
    }
    panic!("Should not happen")
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long() - 1;
    let k = input.read_int();
    let (cycle_start, cycle_len, cycle_offsets) = find_cycle(k);
    let start = cycle_start;
    let mid = mult(n / cycle_offsets.len() as i64, cycle_len);
    let finish = cycle_offsets[(n % cycle_offsets.len() as i64) as usize];
    let result = add(add(start, mid), finish);
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
