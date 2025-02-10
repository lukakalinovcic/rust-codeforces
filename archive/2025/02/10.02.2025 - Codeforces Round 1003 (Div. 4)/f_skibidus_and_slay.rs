//{"name":"F. Skibidus and Slay","group":"Codeforces - Codeforces Round 1003 (Div. 4)","url":"https://codeforces.com/contest/2065/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n3\n1 2 3\n1 3\n2 3\n4\n3 1 1 3\n1 2\n2 3\n4 2\n4\n2 4 4 2\n1 2\n2 3\n3 4\n13\n1 4 4 7 4 7 1 1 7 11 11 11 11\n1 2\n2 3\n3 4\n4 5\n4 6\n2 7\n7 8\n2 9\n6 10\n5 11\n11 12\n10 13\n","output":"000\n1010\n0001\n1001001000100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSkibidusAndSlay"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input
        .read_int_vec(n)
        .into_iter()
        .map(|a| a - 1)
        .collect::<Vec<_>>();
    let mut result = vec![b'0'; n];
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(n - 1) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    rec(&adj, 0, 0, &a, &mut result);
    out.print_line(String::from_utf8(result).unwrap());
}

fn rec(adj: &Vec<Vec<usize>>, u: usize, p: usize, a: &Vec<i32>, result: &mut Vec<u8>) {
    let mut kids = vec![];
    for &v in &adj[u] {
        if v == p {
            continue;
        }
        if a[u] == a[v] || a[p] == a[v] {
            result[a[v] as usize] = b'1';
        }
        kids.push(a[v]);
        rec(adj, v, u, a, result);
    }
    kids.sort();
    for i in 1..kids.len() {
        if kids[i - 1] == kids[i] {
            result[kids[i] as usize] = b'1';
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
