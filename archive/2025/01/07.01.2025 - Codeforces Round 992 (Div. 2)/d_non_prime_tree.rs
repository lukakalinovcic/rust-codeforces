//{"name":"D. Non Prime Tree","group":"Codeforces - Codeforces Round 992 (Div. 2)","url":"https://codeforces.com/contest/2040/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n1 2\n2 3\n2 4\n3 5\n7\n1 2\n1 3\n2 4\n3 5\n3 6\n3 7\n","output":"2 10 1 6 5\n8 7 12 1 4 6 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNonPrimeTree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = Vec<bool>;

const MAX: usize = 2 * 100000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, sieve: &mut PreCalc) {
    let n: usize = input.read();
    let mut adj = vec![vec![]; n];
    for (a, b) in input.read_int_pair_vec(n - 1) {
        let (a, b) = (a - 1, b - 1);
        adj[a as usize].push(b);
        adj[b as usize].push(a);
    }
    let mut seen = vec![false; n];
    let mut bottom: i32 = 1;
    let mut top: i32 = 2 * n as i32;
    let mut curr = vec![0];
    let mut result = vec![0; n];
    let mut layer = 0;
    while !curr.is_empty() {
        let mut next = vec![];
        for u in curr {
            seen[u] = true;
            let mut parent = -1;
            for v in adj[u as usize].iter() {
                let v = *v;
                if seen[v as usize] {
                    parent = v;
                } else {
                    next.push(v as usize);
                }
            }

            if layer % 2 == 0 {
                while parent != -1 && sieve[(bottom - result[parent as usize]).abs() as usize] {
                    bottom += 1;
                }
                result[u as usize] = bottom;
                bottom += 1;
            } else {
                while parent != -1 && sieve[(top - result[parent as usize]).abs() as usize] {
                    top -= 1;
                }
                result[u as usize] = top;
                top -= 1;
            }
        }

        layer += 1;
        curr = next
    }
    out.print_iter(result.into_iter());
    out.put(b'\n');
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = vec![true; 2 * MAX + 1];
    pre_calc[1] = false;
    for i in 2.. {
        if !pre_calc[i] {
            continue;
        }
        if i * i >= pre_calc.len() {
            break;
        }
        let mut j = i * i;
        while j < pre_calc.len() {
            pre_calc[j] = false;
            j += i;
        }
    }

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
