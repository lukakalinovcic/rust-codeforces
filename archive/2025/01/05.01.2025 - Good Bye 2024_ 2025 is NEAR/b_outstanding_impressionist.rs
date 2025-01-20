//{"name":"B. Outstanding Impressionist","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n1 1\n1 1\n4\n1 3\n1 3\n1 3\n1 3\n6\n3 6\n2 2\n1 2\n1 1\n3 4\n2 2\n7\n3 4\n4 4\n4 4\n1 3\n2 5\n1 4\n2 2\n3\n4 5\n4 4\n5 5\n","output":"00\n1111\n100110\n1001111\n011\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOutstandingImpressionist"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn update(a: &mut [i32], mut i: i32, delta: i32) {
    while (i as usize) < a.len() {
        a[i as usize] += delta;
        i += i & -i;
    }
}

fn query(a: &[i32], mut i: i32) -> i32 {
    let mut result = 0;
    while i > 0 {
        result += a[i as usize];
        i -= i & -i;
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int() as usize;
    let a: Vec<_> = (0..n)
        .map(|_| (input.read_int(), input.read_int()))
        .collect();
    let mut busy_count = vec![0; 2 * n + 1];
    let mut busy_loga = vec![0; 2 * n + 1];
    for r in a.iter() {
        let (lo, hi) = *r;
        if lo == hi {
            busy_count[lo as usize] += 1;
            if busy_count[lo as usize] == 1 {
                update(&mut busy_loga, lo, 1);
            }
        }
    }
    for r in a.iter() {
        let mut result = 0;
        let (lo, hi) = *r;
        if lo == hi {
            if busy_count[lo as usize] == 1 {
                result = 1;
            }
        } else {
            let busy_cnt = query(&busy_loga, hi) - query(&busy_loga, lo - 1);
            if busy_cnt != hi - lo + 1 {
                result = 1;
            }
        }
        out.print(result);
    }
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
