//{"name":"G3. Yunli's Subarray Queries (extreme version)","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/G3","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n7 2 4\n1 2 3 2 1 2 3\n4 6\n1 7\n2 7\n3 7\n8 4 2\n4 3 1 1 2 4 3 2\n3 6\n1 5\n5 4 2\n4 5 1 2 3\n1 4\n1 5\n10 4 8\n2 3 6 5 8 9 8 10 10 1\n2 7\n6 10\n1 9\n1 6\n3 9\n4 10\n2 10\n1 8\n10 7 4\n3 4 5 3 4 5 9 10 8 9\n1 9\n2 10\n1 10\n2 9\n","output":"1\n3\n3\n3\n2\n7\n2\n4\n8\n6\n28\n7\n16\n20\n32\n19\n18\n15\n26\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G3YunlisSubarrayQueriesExtremeVersion"}}}

use std::collections::BTreeSet;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

fn window_modes(a: Vec<i32>, k: usize) -> Vec<i32> {
    let mut result = vec![];
    let mut cnt = HashMap::<i32, i32>::new();
    let mut pq = BTreeSet::<(i32, i32)>::new();
    for i in 0..a.len() {
        {
            let a = a[i];
            let c = cnt.entry(a).or_insert(0);
            pq.remove(&(*c, a));
            *c += 1;
            pq.insert((*c, a));
        }
        if i >= k - 1 {
            result.push(pq.last().unwrap().0);

            let a = a[i - (k - 1)];
            let c = cnt.entry(a).or_insert(0);
            pq.remove(&(*c, a));
            *c -= 1;
            pq.insert((*c, a));
        }
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read();
    let q = input.read();
    let a = input.read_int_vec(n);
    let a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| a - i as i32)
        .collect();
    let a = window_modes(a, k)
        .into_iter()
        .map(|mode| k as i32 - mode)
        .collect();
    let q = input
        .read_int_pair_vec(q)
        .into_iter()
        .map(|(l, r)| (l, r - k as i32 + 1))
        .collect();
    let query_results = doit(a, q);
    for qr in query_results {
        out.print_line(qr);
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

#[derive(Default, Debug, Clone)]
struct Borders {
    prev: i32,
    next: i32,
}

#[derive(Default, Debug, Clone)]
struct Query {
    l: i32, // 0-indexed inclusive
    r: i32, // 0-indexed inclusive
    i: usize,
}

const INF: i32 = 1000000000;

fn compute_borders(a: &Vec<i32>) -> Vec<Borders> {
    let n = a.len();
    let mut b = vec![Borders::default(); n + 1];
    let mut s = vec![(-INF, 0)];
    for i in 1..=n {
        while a[i - 1] < s.last().unwrap().0 {
            s.pop();
        }
        b[i].prev = s.last().unwrap().1;
        s.push((a[i - 1], i as i32));
    }
    let mut s = vec![(-INF, n as i32 + 1)];
    for i in (1..=n).rev() {
        while a[i - 1] <= s.last().unwrap().0 {
            s.pop();
        }
        b[i].next = s.last().unwrap().1;
        s.push((a[i - 1], i as i32));
    }
    b
}

#[derive(Default, Clone, Debug)]
struct StaticSparseRangeSumItem {
    position: i32,
    value: i64,
}

#[derive(Default, Clone, Debug)]
struct StaticSparseRangeSum {
    items: Vec<StaticSparseRangeSumItem>,
    prefix_sum: Vec<i64>,
}

impl StaticSparseRangeSum {
    fn from_items(mut items: Vec<StaticSparseRangeSumItem>) -> Self {
        items.sort_by_key(|item| item.position);
        let mut prefix_sum = vec![0; items.len() + 1];
        for i in 0..items.len() {
            prefix_sum[i + 1] = prefix_sum[i] + items[i].value
        }
        Self { items, prefix_sum }
    }

    fn merge(a: &StaticSparseRangeSum, b: &StaticSparseRangeSum) -> Self {
        let mut i = 0;
        let mut j = 0;
        let mut items: Vec<StaticSparseRangeSumItem> = vec![];
        let mut prefix_sum = vec![0];
        while i < a.items.len() || j < b.items.len() {
            let item = if j == b.items.len()
                || i < a.items.len() && a.items[i].position < b.items[j].position
            {
                let item = a.items[i].clone();
                i += 1;
                item
            } else {
                let item = b.items[j].clone();
                j += 1;
                item
            };
            if !items.is_empty() && (*items.last().unwrap()).position == item.position {
                items.last_mut().unwrap().value += item.value;
                *prefix_sum.last_mut().unwrap() += item.value;
            } else {
                prefix_sum.push(prefix_sum.last().unwrap() + item.value);
                items.push(item);
            }
        }
        Self { items, prefix_sum }
    }

    // both inclusive
    fn query(&self, lo: i32, hi: i32) -> i64 {
        let hi = hi + 1;
        let (Ok(j) | Err(j)) = self.items.binary_search_by(|item| item.position.cmp(&hi));
        let (Ok(i) | Err(i)) = self.items.binary_search_by(|item| item.position.cmp(&lo));
        self.prefix_sum[j] - self.prefix_sum[i]
    }
}

fn rectangle_query(
    tree: &mut Vec<StaticSparseRangeSum>,
    i: usize,
    lo: usize,
    hi: usize,
    x1: usize,
    x2: usize,
    y1: i32,
    y2: i32,
) -> i64 {
    if hi < x1 || lo > x2 {
        0
    } else if lo >= x1 && hi <= x2 {
        tree[i].query(y1, y2)
    } else {
        let mid = (lo + hi) / 2;
        let l = rectangle_query(tree, 2 * i, lo, mid, x1, x2, y1, y2);
        let r = rectangle_query(tree, 2 * i + 1, mid + 1, hi, x1, x2, y1, y2);
        l + r
    }
}

// b.prev >= l && b.next <= r
fn process_type0(a: &Vec<i32>, b: &Vec<Borders>, q: &Vec<Query>) -> Vec<i64> {
    let n = a.len();
    let mut leaf_items = vec![vec![]; n + 1];
    for i in 1..=n {
        let a = a[i - 1] as i64;
        let prev_cnt = (i as i32 - b[i].prev) as i64;
        let next_cnt = (b[i].next - i as i32) as i64;
        let value = a * prev_cnt * next_cnt;
        leaf_items[b[i].prev as usize].push(StaticSparseRangeSumItem {
            position: b[i].next,
            value,
        });
    }
    let ts = (n + 1).next_power_of_two();
    let mut tree = vec![StaticSparseRangeSum::default(); 2 * ts];
    for i in 0..=n {
        tree[ts + i] = StaticSparseRangeSum::from_items(leaf_items[i].drain(..).collect());
    }
    for i in (1..ts).rev() {
        tree[i] = StaticSparseRangeSum::merge(&tree[2 * i], &tree[2 * i + 1]);
    }
    let mut query_results = vec![0; q.len()];
    for query in q {
        query_results[query.i] = rectangle_query(
            &mut tree,
            1,
            0,
            ts - 1,
            query.l as usize,
            query.r as usize - 1,
            query.l + 1,
            query.r,
        );
    }
    query_results
}

type ResultAndValueToAdd = (i64, i64);
type AddValueCnt = i32;

struct MySpec {}
impl LazySegTreeSpec<ResultAndValueToAdd, AddValueCnt> for &MySpec {
    fn identity(&self) -> ResultAndValueToAdd {
        (0, 0)
    }

    fn op(&self, a: &ResultAndValueToAdd, b: &ResultAndValueToAdd) -> ResultAndValueToAdd {
        (a.0 + b.0, a.1 + b.1)
    }

    fn compose(&self, a: &AddValueCnt, b: &AddValueCnt) -> AddValueCnt {
        a + b
    }

    fn update(&self, t: &ResultAndValueToAdd, u: &AddValueCnt) -> ResultAndValueToAdd {
        (t.0 + t.1 * (*u) as i64, t.1)
    }
}

// if reversed = 0: b.next > r
// else: b.prev >= l && b.next > r
fn process_type1(a: &Vec<i32>, q: &Vec<Query>, reversed: i32) -> Vec<i64> {
    let n = a.len();
    let mut s = vec![(-INF, 0)];
    let mut q = q.clone();
    q.sort_by_key(|query| query.r);
    let mut query_results = vec![0; q.len()];
    let mut next_query = 0;
    let tree_spec = MySpec {};
    let mut tree = LazySegTree::new(&tree_spec, n + 1);
    for i in 1..=n {
        while a[i - 1] - reversed < s.last().unwrap().0 {
            let (_, pos) = s.pop().unwrap();
            tree.set(pos as usize, (0, 0));
        }
        {
            let a = a[i - 1] as i64;
            let prev_cnt = (i as i32 - s.last().unwrap().1) as i64;
            s.push((a as i32, i as i32));
            tree.set(i as usize, (a * prev_cnt, a * prev_cnt));
        }
        while next_query < q.len() && q[next_query].r == i as i32 {
            let query = &q[next_query];
            next_query += 1;
            let (Ok(j) | Err(j)) = s.binary_search_by(|(_, pos)| pos.cmp(&(query.l)));
            if reversed == 0 {
                let (a, pos) = s[j];
                if pos >= query.l {
                    let a = a as i64;
                    let prev_cnt = (pos - query.l + 1) as i64;
                    let next_cnt = (query.r + 1 - pos) as i64;
                    query_results[query.i] += a * prev_cnt * next_cnt;
                }
            }
            if j + 1 < s.len() {
                let lo_pos = s[j + 1].1 as usize;
                query_results[query.i] += tree.product(lo_pos, n + 1).0;
            }
        }
        tree.update(0, n + 1, 1);
    }
    query_results
}

// b.prev < l && b.next <= r
fn process_type2(a: &Vec<i32>, q: &Vec<Query>) -> Vec<i64> {
    let n = a.len();
    let mut a = a.clone();
    a.reverse();
    let q = q
        .iter()
        .map(|q| Query {
            l: n as i32 - q.r + 1,
            r: n as i32 - q.l + 1,
            i: q.i,
        })
        .collect();
    process_type1(&a, &q, 1)
}

fn doit(a: Vec<i32>, q: Vec<(i32, i32)>) -> Vec<i64> {
    let b = compute_borders(&a);
    let mut query_results = vec![[0, 0, 0]; q.len()];
    let mut q = q
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| Query { l, r, i })
        .collect::<Vec<_>>();
    for (i, qr) in process_type0(&a, &b, &q).into_iter().enumerate() {
        query_results[i][0] += qr;
    }
    for (i, qr) in process_type1(&a, &mut q, 0).into_iter().enumerate() {
        query_results[i][1] += qr;
    }
    for (i, qr) in process_type2(&a, &mut q).into_iter().enumerate() {
        query_results[i][2] += qr;
    }
    query_results
        .into_iter()
        .map(|[a, b, c]| a + b + c)
        .collect()
}

//START MAIN
mod tester;

fn brute(a: Vec<i32>, q: Vec<(i32, i32)>) -> Vec<i64> {
    let b = compute_borders(&a);
    let mut query_results = vec![0; q.len()];
    let q = q
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| Query { l, r, i })
        .collect::<Vec<_>>();
    for query in q {
        let (l, r) = (query.l, query.r);
        for i in l as usize..=r as usize {
            let (pr, nx) = (b[i].prev, b[i].next);
            let (pr, nx) = if pr >= l - 1 && nx <= r + 1 {
                (pr, nx)
            } else if pr >= l - 1 && nx > r + 1 {
                (pr, r + 1)
            } else if pr < l - 1 && nx <= r + 1 {
                (l - 1, nx)
            } else {
                (l - 1, r + 1)
            };
            let a = a[i - 1] as i64;
            let prev_cnt = (i as i32 - pr) as i64;
            let next_cnt = (nx - i as i32) as i64;
            query_results[query.i] += a * prev_cnt * next_cnt;
        }
    }
    query_results
}
fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let maxn = 20000;
        let n = rng.gen_range(maxn..=maxn);
        let q = 20000;
        let a = (0..n).map(|_| rng.gen_range(0..=10)).collect::<Vec<_>>();
        let lr = (0..q)
            .map(|_| {
                let mut l = rng.gen_range(1..=n);
                let mut r = rng.gen_range(1..=n);
                if r < l {
                    (l, r) = (r, l)
                }
                (l, r)
            })
            .collect::<Vec<_>>();

        let t0 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let s1 = doit(a.clone(), lr.clone());
        let t1 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        eprintln!("{:?}", t1 - t0);
        let s2 = brute(a.clone(), lr.clone());
        let t2 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        eprintln!("{:?} {:?}", t1 - t0, t2 - t1);
        if s1 != s2 {
            eprintln!("Mismatch at {n} {q}\n{a:?}\n{lr:?}\ndoit = {s1:?}\nbrute = {s2:?}");
            break;
        }
    }
}
//END MAIN
