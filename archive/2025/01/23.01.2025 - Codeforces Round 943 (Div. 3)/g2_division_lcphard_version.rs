//{"name":"G2. Division + LCP (hard version)","group":"Codeforces - Codeforces Round 943 (Div. 3)","url":"https://codeforces.com/contest/1968/problem/G2","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3 1 3\naba\n3 2 3\naaa\n7 1 5\nabacaba\n9 1 6\nabababcab\n10 1 10\naaaaaaawac\n9 1 9\nabafababa\n7 2 7\nvvzvvvv\n","output":"3 1 0\n1 1\n7 3 1 1 0\n9 2 2 2 0 0\n10 3 2 1 1 1 1 1 0 0\n9 3 2 1 1 0 0 0 0\n2 2 1 1 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G2DivisionLCPHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

fn counting_sort_indices(ind: &Vec<usize>, val: &Vec<i32>, max_val: i32) -> Vec<usize> {
    let n = ind.len();
    let m = max_val as usize + 1;
    let mut c = vec![0; m];
    for i in 0..n {
        c[val[ind[i]] as usize] += 1;
    }
    for i in 1..m {
        c[i] += c[i - 1];
    }
    let mut result = vec![0; n];
    for i in (0..n).rev() {
        c[val[ind[i]] as usize] -= 1;
        let pos = c[val[ind[i]] as usize];
        result[pos] = ind[i];
    }
    result
}

fn cyclic_suffix_array(a: &Vec<i32>, max_val: i32) -> Vec<usize> {
    let n = a.len();

    let p = (0..n).collect::<Vec<_>>();
    let mut p = counting_sort_indices(&p, a, max_val);

    let mut c = vec![0; n];
    let mut class = 0;
    for i in 0..n {
        if i > 0 && a[p[i]] != a[p[i - 1]] {
            class += 1;
        }
        c[p[i]] = class;
    }

    let mut h = 0;
    while (1 << h) < n {
        p = p
            .into_iter()
            .map(|x| {
                if x >= (1 << h) {
                    x - (1 << h)
                } else {
                    x + n - (1 << h)
                }
            })
            .collect::<Vec<_>>();
        p = counting_sort_indices(&p, &c, class);

        class = 0;
        let mut new_c = vec![0; n];
        for i in 0..n {
            if i > 0 {
                let pi_a = p[i];
                let pi_b = (pi_a + (1 << h)) % n;
                let pj_a = p[i - 1];
                let pj_b = (pj_a + (1 << h)) % n;
                if (c[pi_a], c[pi_b]) != (c[pj_a], c[pj_b]) {
                    class += 1;
                }
            }
            new_c[p[i]] = class;
        }
        c = new_c;
        h += 1;
    }
    p
}

struct MySpec {}
impl LazySegTreeSpec<Option<i32>, ()> for &MySpec {
    fn identity(&self) -> Option<i32> {
        None
    }

    fn op(&self, a: &Option<i32>, b: &Option<i32>) -> Option<i32> {
        if a.is_none() {
            *b
        } else {
            *a
        }
    }

    fn compose(&self, _old: &(), _new: &()) -> () {
        ()
    }

    fn update(&self, t: &Option<i32>, _u: &()) -> Option<i32> {
        *t
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let l: usize = input.read();
    let r: usize = input.read();
    let s = input.read_line().into_bytes();
    let mut a = s.iter().map(|x| *x as i32).collect::<Vec<_>>();
    a.push(127);
    let p = cyclic_suffix_array(&a, 128);

    let mut matching = vec![0; n];
    matching[0] = n;
    let zero = p.iter().position(|x| *x == 0).unwrap();
    {
        let mut curr_match = 0;
        for i in 0..zero {
            while p[i] + curr_match < n && s[p[i] + curr_match] == s[curr_match] {
                curr_match += 1;
            }
            matching[p[i]] = curr_match;
        }
    }
    {
        let mut curr_match = 0;
        for i in (zero + 1..n).rev() {
            while p[i] + curr_match < n && s[p[i] + curr_match] == s[curr_match] {
                curr_match += 1;
            }
            matching[p[i]] = curr_match;
        }
    }

    let tree_spec = MySpec {};
    let mut tree = LazySegTree::new(&tree_spec, n + 1);
    tree.init(|i| Some(i as i32));

    let mut prev_jumps = n;
    let mut result = vec![0; n + 1];
    for lcp in 1..=n {
        let mut x = 0;
        let mut jumps = 0;
        while x < n {
            while x < n && matching[x as usize] < lcp {
                tree.set(x, None);
                x = tree.product(x + 1, n + 1).unwrap() as usize;
            }
            if x < n {
                jumps += 1;
                x += lcp;
            }
        }
        while prev_jumps > jumps {
            result[prev_jumps] = lcp - 1;
            prev_jumps -= 1;
        }
        result[jumps] = lcp;
    }

    out.print_iter(result[l..=r].iter());
    out.print_line("");
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
