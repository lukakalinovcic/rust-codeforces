//{"name":"E2. Eliminating Balls With Merging (Hard Version)","group":"Codeforces - Codeforces Round 965 (Div. 2)","url":"https://codeforces.com/contest/1998/problem/E2","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n5 1\n1 2 3 2 1\n7 1\n4 5 1 2 1 4 5\n11 1\n1 2 3 1 1 9 3 2 4 1 3\n","output":"1 1 2 2 3\n1 1 1 1 1 3 4\n1 1 2 2 2 1 1 1 3 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2EliminatingBallsWithMergingHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

#[derive(Clone, Debug)]
struct Node {
    parent: usize,
    lt: usize,
    rt: usize,
    lo: usize,
    hi: usize,
    threshold: i64,
    active: bool,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _x = input.read_size();
    let a = input.read_long_vec(n);
    let results = doit(a);
    out.print_line(results);
}

fn doit(a: Vec<i64>) -> Vec<i32> {
    let n = a.len();
    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }
    let mut nodes = vec![
        Node {
            parent: n,
            lt: n,
            rt: n,
            lo: 0,
            hi: 0,
            threshold: 0,
            active: false,
        };
        n
    ];
    let mut s: Vec<(i64, usize)> = vec![];
    for i in 0..n {
        let mut last_pop = n;
        while !s.is_empty() && s.last().unwrap().0 <= a[i] {
            let (_, j) = s.pop().unwrap();
            last_pop = j;
        }
        if last_pop != n {
            nodes[i].lt = last_pop;
            nodes[last_pop].parent = i;
        }
        if !s.is_empty() {
            let p = s.last().unwrap().1;
            nodes[p].rt = i;
            nodes[i].parent = p;
        }
        s.push((a[i], i));
    }
    let mut order = vec![];
    postorder(s[0].1, &nodes, &mut order);
    for i in 0..n {
        let u = order[i];
        let (lt, rt) = (nodes[u].lt, nodes[u].rt);
        nodes[u].lo = if lt == n { u } else { nodes[lt].lo };
        nodes[u].hi = if rt == n { u } else { nodes[rt].hi };
        nodes[u].threshold = if i == n - 1 { 0 } else { a[nodes[u].parent] };
    }
    let mut watch_list = vec![vec![]; n];
    let mut num_active = activate_subtree(order[n - 1], &prefix, &mut nodes, &mut watch_list);
    let mut results = vec![0; n];
    for i in (0..n).rev() {
        results[i] = num_active;
        if nodes[i].active {
            nodes[i].active = false;
            num_active -= 1;
        }
        let lt = nodes[i].lt;
        if lt != n {
            let p = nodes[i].parent;
            if p != n {
                nodes[p].rt = lt;
                nodes[lt].parent = p;
                nodes[lt].threshold = a[p];
            } else {
                nodes[lt].parent = n;
                nodes[lt].threshold = 0;
                num_active += activate_subtree(lt, &prefix, &mut nodes, &mut watch_list);
            }
            if prefix[i] - prefix[nodes[lt].lo] < nodes[lt].threshold {
                num_active -= deactivate_subtree(lt, &mut nodes);
            }
            if nodes[lt].active {
                let j = bsearch(nodes[lt].lo, i - 1, &prefix, nodes[lt].threshold);
                if j > lt {
                    watch_list[j].push(lt);
                }
            }
        }
        for u in watch_list[i].drain(..) {
            let sum = prefix[i] - prefix[nodes[u].lo];
            if sum < nodes[u].threshold {
                num_active -= deactivate_subtree(u, &mut nodes);
            }
        }
    }
    results
}

fn bsearch(a: usize, b: usize, prefix: &[i64], threshold: i64) -> usize {
    let mut lo = a;
    let mut hi = b;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        let sum = prefix[mid] - prefix[a];
        if sum < threshold {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

fn deactivate_subtree(u: usize, nodes: &mut Vec<Node>) -> i32 {
    if !nodes[u].active {
        return 0;
    }
    let mut deactivated = 1;
    nodes[u].active = false;
    if nodes[u].lt != nodes.len() {
        deactivated += deactivate_subtree(nodes[u].lt, nodes);
    }
    if nodes[u].rt != nodes.len() {
        deactivated += deactivate_subtree(nodes[u].rt, nodes);
    }
    deactivated
}

fn activate_subtree(
    u: usize,
    prefix: &[i64],
    nodes: &mut Vec<Node>,
    watch_list: &mut Vec<Vec<usize>>,
) -> i32 {
    let (lo, hi, thresh) = (nodes[u].lo, nodes[u].hi, nodes[u].threshold);
    if nodes[u].active || prefix[hi + 1] - prefix[lo] < thresh {
        return 0;
    }
    let mut activated = 1;
    nodes[u].active = true;
    let j = bsearch(lo, hi, &prefix, thresh);
    if j > u {
        watch_list[j].push(u);
    }
    if nodes[u].lt != nodes.len() {
        activated += activate_subtree(nodes[u].lt, prefix, nodes, watch_list);
    }
    if nodes[u].rt != nodes.len() {
        activated += activate_subtree(nodes[u].rt, prefix, nodes, watch_list);
    }
    activated
}

fn postorder(i: usize, nodes: &Vec<Node>, out: &mut Vec<usize>) {
    if nodes[i].lt != nodes.len() {
        postorder(nodes[i].lt, nodes, out);
    }
    if nodes[i].rt != nodes.len() {
        postorder(nodes[i].rt, nodes, out);
    }
    out.push(i);
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

fn brute(a: Vec<i64>) -> Vec<i32> {
    let n = a.len();
    let mut result = vec![];
    for j in 1..=n {
        let mut lt = vec![n; n];
        let mut rt = vec![n; n];
        let mut s: Vec<(i64, usize)> = vec![];
        for i in 0..j {
            while !s.is_empty() && s.last().unwrap().0 <= a[i] {
                let (_, j) = s.pop().unwrap();
                lt[i] = j;
            }
            if !s.is_empty() {
                rt[s.last().unwrap().1] = i;
            }
            s.push((a[i], i));
        }
        result.push(rec(s[0].1, 0, &a, &lt, &rt).1);
    }
    result
}

fn rec(i: usize, threshold: i64, a: &[i64], lt: &[usize], rt: &[usize]) -> (i64, i32) {
    let mut sum = a[i];
    let mut cnt = 1;
    if lt[i] != a.len() {
        let (s, c) = rec(lt[i], a[i], a, lt, rt);
        sum += s;
        cnt += c;
    }
    if rt[i] != a.len() {
        let (s, c) = rec(rt[i], a[i], a, lt, rt);
        sum += s;
        cnt += c;
    }
    if sum < threshold {
        cnt = 0;
    }
    (sum, cnt)
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(2..=100);
        let a = (0..n)
            .map(|_| rng.gen_range(1..=100) as i64)
            .collect::<Vec<_>>();
        let x = doit(a.clone());
        let y = brute(a.clone());
        if x != y {
            eprintln!("Mismatch at {a:?}   {x:?} != {y:?}");
            break;
        }
    }
}
//END MAIN
