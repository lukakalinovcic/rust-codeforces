//{"name":"F. Nene and the Passing Game","group":"Codeforces - Codeforces Round 939 (Div. 2)","url":"https://codeforces.com/contest/1956/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n2\n1 1\n1 1\n2\n1 1\n2 2\n3\n1 3\n1 3\n1 3\n5\n1 1\n2 2\n1 5\n2 2\n1 1\n6\n1 2\n5 5\n2 3\n2 3\n2 2\n1 2\n","output":"2\n2\n2\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNeneAndThePassingGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn find(parent: &mut Vec<i32>, x: i32) -> i32 {
    let p = parent[x as usize];
    if p == x {
        p
    } else {
        let p = find(parent, p);
        parent[x as usize] = p;
        p
    }
}

fn union(parent: &mut Vec<i32>, x: i32, y: i32) {
    let x = find(parent, x);
    let y = find(parent, y);
    parent[y as usize] = x;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let mut lt_next = vec![-1; n as usize];
    let mut rt_next = vec![-1; n as usize];
    let mut new_edge_span = vec![];
    for i in 0..n {
        let l = input.read_int();
        let r = input.read_int();
        {
            let lo = (i + l).min(n - 1);
            let hi = (i + r).min(n - 1);
            lt_next[lo as usize] = lt_next[lo as usize].max(hi - lo);
            new_edge_span.push((lo, i, hi));
        }
        {
            let lo = (i - r).max(0);
            let hi = (i - l).max(0);
            rt_next[lo as usize] = rt_next[lo as usize].max(hi - lo);
            new_edge_span.push((lo, i, hi));
        }
    }
    new_edge_span.sort();
    let mut new_edge_span_start = vec![0; n as usize + 1];
    for i in 1..=n as usize {
        new_edge_span_start[i] = new_edge_span_start[i - 1];
        while new_edge_span_start[i] < new_edge_span.len()
            && new_edge_span[new_edge_span_start[i] as usize].0 < i as i32
        {
            new_edge_span_start[i] += 1;
        }
    }
    for i in 1..n {
        let i = i as usize;
        lt_next[i] = lt_next[i].max(lt_next[i - 1] - 1);
        rt_next[i] = rt_next[i].max(rt_next[i - 1] - 1);
    }

    let mut parent = (0..2 * n).collect::<Vec<_>>();
    let mut prev = -1;
    for curr in 0..n {
        let lt_curr = lt_next[curr as usize];
        let rt_curr = rt_next[curr as usize];
        if lt_curr == -1 || rt_curr == -1 {
            continue;
        }
        if prev != -1 {
            let lt_prev = lt_next[prev as usize];
            let rt_prev = rt_next[prev as usize];
            if prev + lt_prev.max(rt_prev) >= curr {
                union(&mut parent, n + prev, n + curr);
            }
        }
        for (_, x, ttl) in &new_edge_span[new_edge_span_start[(prev + 1) as usize] as usize
            ..new_edge_span_start[curr as usize + 1] as usize]
        {
            if *ttl >= curr {
                union(&mut parent, *x, n + curr);
            }
        }
        prev = curr;
    }

    for i in 0..n {
        find(&mut parent, i);
    }
    parent.truncate(n as usize);
    parent.sort();
    parent.dedup();
    out.print_line(parent.len());
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
