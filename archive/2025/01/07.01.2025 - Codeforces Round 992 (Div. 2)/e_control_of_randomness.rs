//{"name":"E. Control of Randomness","group":"Codeforces - Codeforces Round 992 (Div. 2)","url":"https://codeforces.com/contest/2040/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 4\n1 2\n2 3\n2 4\n2 0\n3 0\n4 0\n3 1\n12 10\n1 2\n2 3\n2 4\n1 5\n5 6\n6 7\n6 8\n6 9\n8 10\n10 11\n10 12\n6 0\n9 0\n10 0\n11 0\n3 1\n7 1\n10 1\n12 1\n12 2\n11 12\n","output":"1\n6\n6\n2\n4\n9\n8\n15\n2\n3\n6\n9\n5\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EControlOfRandomness"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let q: usize = input.read();
    let mut adj = vec![vec![]; n];
    for (a, b) in input.read_int_pair_vec(n - 1) {
        let (a, b) = (a - 1, b - 1);
        adj[a as usize].push(b);
        adj[b as usize].push(a);
    }
    let mut parent = vec![0; n];
    let mut queue = VecDeque::new();
    let mut num_children = vec![0; n];
    queue.push_back((0i32, -1));
    while !queue.is_empty() {
        let (u, p) = queue.pop_front().unwrap();
        for v in adj[u as usize].iter() {
            let v = *v;
            if v == p {
                parent[u as usize] = p;
            } else {
                queue.push_back((v, u));
                num_children[u as usize] += 1;
            }
        }
    }
    for _ in 0..q {
        let mut v = input.read_int() - 1;
        let coins = input.read_int();
        let mut a = vec![];
        let mut steps = 0;
        for i in 1.. {
            if v == 0 {
                break;
            }
            steps += 1;
            if i % 2 == 0 {
                a.push(num_children[v as usize]);
            }
            v = parent[v as usize];
        }
        a.sort();
        let a = &a[0..(a.len() as i32 - coins).max(0) as usize];
        let mut result = steps;
        for x in a {
            result += 2 * *x;
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
