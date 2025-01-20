//{"name":"E. C+K+S","group":"Codeforces - Codeforces Round 980 (Div. 2)","url":"https://codeforces.com/contest/2024/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4 2\n1 0 0 1\n4\n1 2\n2 3\n3 4\n4 1\n1 0 0 1\n4\n1 3\n3 2\n2 4\n4 1\n3 3\n0 0 0\n3\n1 2\n2 3\n3 1\n1 1 0\n3\n1 2\n2 3\n3 1\n4 2\n1 1 1 1\n4\n1 2\n2 3\n3 4\n4 1\n0 0 0 0\n6\n1 2\n2 1\n1 3\n3 1\n1 4\n4 1\n","output":"YES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECKS"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn kmp_cyclic_match<T: Eq + Clone + std::fmt::Debug>(a: Vec<T>, b: Vec<T>, separator: T) -> bool {
    let a_len = a.len();
    let s = vec![a, vec![separator], b.clone(), b].concat();
    let n = s.len();
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1] as usize;
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1] as usize;
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j as i32;
        if i > a_len && pi[i] as usize == a_len {
            return true;
        }
    }
    false
}

fn read_graph(n: usize, input: &mut Input) -> (Vec<Vec<usize>>, Vec<i32>) {
    let outgoing = input.read_int_vec(n);
    let mut adj = vec![vec![]; n];
    let m = input.read();
    for (a, b) in input.read_int_pair_vec(m) {
        let (a, b) = (a as usize - 1, b as usize - 1);
        adj[a].push(b);
    }
    (adj, outgoing)
}

fn label_graph(n: usize, adj: &Vec<Vec<usize>>, k: i32) -> Vec<i32> {
    let mut label = vec![-1; n];
    let mut s = vec![];
    label[0] = 0;
    s.push(0);
    while !s.is_empty() {
        let u = s.pop().unwrap();
        let next_label = (label[u] + 1) % k;
        for v in &adj[u] {
            let v = *v;
            if label[v] == -1 {
                label[v] = next_label;
                s.push(v);
            }
        }
    }
    label
}

fn gen_cnts(
    n: usize,
    k: usize,
    labels: Vec<i32>,
    outgoing: Vec<i32>,
    is_first: bool,
) -> Vec<(i32, i32)> {
    let mut cnts = vec![(0, 0); k];
    for i in 0..n {
        let label = labels[i] as usize;
        let label = if is_first {
            label
        } else {
            if outgoing[i] == 1 {
                (label + 1) % k
            } else {
                (label + k - 1) % k
            }
        };
        if is_first {
            if outgoing[i] == 1 {
                cnts[label].0 += 1;
            } else {
                cnts[label].1 += 1;
            }
        } else {
            if outgoing[i] == 1 {
                cnts[label].1 += 1;
            } else {
                cnts[label].0 += 1;
            }
        }
    }
    cnts
}

fn doit(
    n: usize,
    k: i32,
    adj1: Vec<Vec<usize>>,
    outgoing1: Vec<i32>,
    adj2: Vec<Vec<usize>>,
    outgoing2: Vec<i32>,
) -> String {
    let mut outgoing1_cnt = [0, 0];
    let mut outgoing2_cnt = [0, 0];
    for i in 0..n {
        outgoing1_cnt[outgoing1[i] as usize] += 1;
        outgoing2_cnt[outgoing2[i] as usize] += 1;
    }
    if outgoing1_cnt[1] != outgoing2_cnt[0] {
        return String::from("NO");
    }
    if outgoing1_cnt[1] == 0 || outgoing1_cnt[0] == 0 {
        return String::from("YES");
    }
    let labels1 = label_graph(n, &adj1, k);
    let labels2 = label_graph(n, &adj2, k);
    if kmp_cyclic_match(
        gen_cnts(n, k as usize, labels1, outgoing1, true),
        gen_cnts(n, k as usize, labels2, outgoing2, false),
        (-1, -1),
    ) {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_int();
    let (adj1, outgoing1) = read_graph(n, input);
    let (adj2, outgoing2) = read_graph(n, input);
    out.print_line(doit(n, k, adj1, outgoing1, adj2, outgoing2));
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
