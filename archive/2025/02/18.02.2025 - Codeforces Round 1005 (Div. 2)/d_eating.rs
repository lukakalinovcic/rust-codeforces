//{"name":"D. Eating","group":"Codeforces - Codeforces Round 1005 (Div. 2)","url":"https://codeforces.com/contest/2064/problem/D","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n1 1\n5\n6\n4 4\n1 5 4 11\n8\n13\n16\n15\n10 9\n10 4 3 9 7 4 6 1 9 4\n2\n6\n5\n6\n9\n8\n6\n2\n7\n","output":"1\n0 2 4 2\n0 1 1 1 3 3 1 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DEating"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let a = input.read_int_vec(n);
    let mut prefix = vec![0; n + 1];
    let mut prev = vec![vec![-1; n]; 30];
    for i in 0..n {
        prefix[i + 1] = prefix[i] ^ a[i];
        if i > 0 {
            for b in 0..30 {
                prev[b][i] = prev[b][i - 1];
            }
        }
        let b = 31 - (a[i] as u32).leading_zeros() as i32;
        if b >= 0 {
            for j in 0..=b {
                prev[j as usize][i] = i as i32;
            }
        }
    }
    let mut results = vec![];
    for _ in 0..q {
        let mut x = input.read_int();
        let mut i = n as i32 - 1;
        while i >= 0 && x > 0 {
            let b = (31 - (x as u32).leading_zeros()) as usize;
            let j = prev[b][i as usize];
            x ^= prefix[(i + 1) as usize] ^ prefix[(j + 1) as usize];
            i = j;
            if i < 0 {
                break;
            }
            if a[i as usize] > x {
                break;
            }
            x ^= a[i as usize];
            i -= 1;
        }
        results.push(n as i32 - 1 - i);
    }
    out.print_line(results);
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
