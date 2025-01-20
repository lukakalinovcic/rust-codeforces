//{"name":"G1. Medium Demon Problem (easy version)","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/G1","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n2 1\n5\n2 3 4 5 1\n5\n2 1 4 2 3\n5\n4 1 1 5 4\n10\n4 3 9 1 6 7 9 10 10 3\n","output":"2\n2\n5\n4\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1MediumDemonProblemEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn tree_depth(i: usize, p: usize, adj: &Vec<Vec<usize>>) -> usize {
    let mut result = 1;
    for j in adj[i].iter() {
        if *j == p {
            continue;
        }
        result = result.max(1 + tree_depth(*j, i, adj));
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let r: Vec<usize> = (0..n).map(|_| input.read_int() as usize - 1).collect();

    let mut adj = vec![vec![]; n];
    for (i, j) in r.iter().enumerate() {
        adj[*j].push(i);
    }

    let mut first_seen = vec![n; n];
    let mut cycle_parent = vec![n; n];
    for s in 0..n {
        let mut i = s;
        let mut j = n;
        while first_seen[i] == n {
            first_seen[i] = s;
            j = i;
            i = r[i];
        }
        while first_seen[i] == s && cycle_parent[i] == n {
            cycle_parent[i] = j;
            j = i;
            i = r[i];
        }
        //println!("{s} {first_seen:?} {cycle_parent:?}")
    }

    let mut result = 1;
    for s in 0..n {
        if cycle_parent[s] != n {
            result = result.max(tree_depth(s, cycle_parent[s], &adj));
        }
    }
    out.print_line(1 + result);
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
