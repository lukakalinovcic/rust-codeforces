//{"name":"H. Robin Hood Archery","group":"Codeforces - Codeforces Round 974 (Div. 3)","url":"https://codeforces.com/contest/2014/problem/H","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n3 3\n1 2 2\n1 2\n1 3\n2 3\n5 3\n2 1 2 1 1\n1 2\n1 3\n4 5\n","output":"NO\nNO\nYES\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HRobinHoodArchery"}}}

use std::hash::Hash;
use std::hash::Hasher;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn relabel(a: Vec<i32>) -> (usize, Vec<i32>) {
    let n = a.len();
    let mut sorted = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<Vec<_>>();
    sorted.sort();
    let mut m = 0;
    let mut out = vec![0; n];
    let mut prev = 0;
    for (a, i) in sorted {
        if a != prev {
            prev = a;
            m += 1;
        }
        out[i] = m as i32;
    }
    (m, out)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let (_, a) = relabel(input.read_int_vec(n));
    let mut prefix_xor = vec![0; n + 1];
    for i in 0..n {
        let mut hasher = std::hash::SipHasher::new();
        a[i].hash(&mut hasher);
        prefix_xor[i + 1] = prefix_xor[i] ^ hasher.finish();
    }
    for (a, b) in input.read_int_pair_vec(q) {
        let (a, b) = (a as usize, b as usize);
        if prefix_xor[b] == prefix_xor[a - 1] {
            out.print_line("YES");
        } else {
            out.print_line("NO");
        }
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
