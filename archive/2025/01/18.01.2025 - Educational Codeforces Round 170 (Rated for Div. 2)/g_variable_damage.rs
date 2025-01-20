//{"name":"G. Variable Damage","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/G","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n2 5\n1 4\n1 10\n","output":"0\n8\n19\n"},{"input":"10\n1 9\n1 6\n2 4\n1 8\n1 3\n2 10\n1 3\n1 6\n1 10\n2 6\n","output":"9\n15\n19\n27\n30\n39\n42\n48\n59\n65\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GVariableDamage"}}}

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::mem::swap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read();
    let qs = input.read_int_pair_vec(q);

    let results = doit(qs);
    for result in results {
        out.print_line(result);
    }
}

#[derive(Clone, Default, Debug)]
struct Node {
    min: i32,
    max: i32,
    update_delta: i32,
    update_result_pos: HashMap<i32, i64>,
    update_result_neg: HashMap<i32, i64>,
}

fn propagate(tree: &mut Vec<Node>, i: usize) {
    let update_delta = tree[i].update_delta;
    tree[i].update_delta = 0;
    let mut update_result_pos = HashMap::new();
    let mut update_result_neg = HashMap::new();
    swap(&mut update_result_pos, &mut tree[i].update_result_pos);
    swap(&mut update_result_neg, &mut tree[i].update_result_neg);
    for child in 2 * i..=2 * i + 1 {
        let old_update_delta = tree[child].update_delta;
        tree[child].min += update_delta;
        tree[child].max += update_delta;
        tree[child].update_delta += update_delta;
        for (threshold_delta, result_delta) in update_result_pos.iter() {
            let mut threshold_delta = *threshold_delta;
            let result_delta = *result_delta;
            threshold_delta -= old_update_delta;
            if tree[child].min <= threshold_delta + tree[child].update_delta {
                let extra = threshold_delta + tree[child].update_delta - tree[child].max;
                if extra > 0 {
                    threshold_delta -= extra;
                }
                add_update_result(&mut tree[child], 1, threshold_delta, result_delta);
            }
        }
        for (threshold_delta, result_delta) in update_result_neg.iter() {
            let mut threshold_delta = *threshold_delta;
            let result_delta = *result_delta;
            threshold_delta -= old_update_delta;
            if tree[child].max >= threshold_delta + tree[child].update_delta {
                let extra = tree[child].min - threshold_delta - tree[child].update_delta;
                if extra > 0 {
                    threshold_delta += extra;
                }
                add_update_result(&mut tree[child], -1, threshold_delta, result_delta);
            }
        }
    }
}

fn add_update_result(node: &mut Node, direction: i32, threshold_delta: i32, result_delta: i64) {
    let container = if direction == 1 {
        &mut node.update_result_pos
    } else {
        &mut node.update_result_neg
    };
    match container.entry(threshold_delta) {
        Entry::Occupied(mut e) => {
            *e.get_mut() += result_delta;
        }
        Entry::Vacant(e) => {
            e.insert(result_delta);
        }
    }
}

fn update(
    tree: &mut Vec<Node>,
    i: usize,
    lo: usize,
    hi: usize,
    a: usize,
    b: usize,
    delta: i32,
    result_delta: i64,
) {
    if hi <= a || lo >= b {
        return;
    }
    if lo >= a && hi <= b {
        tree[i].min += delta;
        tree[i].max += delta;
        // for ur in tree[i].update_result.iter_mut() {
        //     ur.threshold_delta += delta;
        // }
        // TODO merge properly, and prune by min/max
        tree[i].update_delta += delta;
        let mut threshold_delta = -tree[i].update_delta;
        if delta == 1 {
            if tree[i].min <= 0 {
                if tree[i].max < 0 {
                    threshold_delta -= -tree[i].max;
                }
                add_update_result(&mut tree[i], 1, threshold_delta, result_delta);
            }
        } else {
            if tree[i].max >= 0 {
                if tree[i].min > 0 {
                    threshold_delta += tree[i].min;
                }
                add_update_result(&mut tree[i], -1, threshold_delta, result_delta);
            }
        }
    } else {
        let mid = (lo + hi) / 2;
        propagate(tree, i);
        update(tree, 2 * i, lo, mid, a, b, delta, result_delta);
        update(tree, 2 * i + 1, mid, hi, a, b, delta, result_delta);
        tree[i].min = tree[2 * i].min.min(tree[2 * i + 1].min);
        tree[i].max = tree[2 * i].max.max(tree[2 * i + 1].max);
    }
}

fn query_result(tree: &mut Vec<Node>, i: usize, lo: usize, hi: usize, x: usize) -> i64 {
    if x < lo || x >= hi {
        return 0;
    }
    if hi - lo == 1 {
        let mut result = 0;
        for (_, result_delta) in &tree[i].update_result_pos {
            result += *result_delta;
        }
        for (_, result_delta) in &tree[i].update_result_neg {
            result += *result_delta;
        }
        result
    } else {
        let mid = (lo + hi) / 2;
        propagate(tree, i);
        let result =
            query_result(tree, 2 * i, lo, mid, x) + query_result(tree, 2 * i + 1, mid, hi, x);
        tree[i].min = tree[2 * i].min.min(tree[2 * i + 1].min);
        tree[i].max = tree[2 * i].max.max(tree[2 * i + 1].max);
        result
    }
}

fn doit(qs: Vec<(i32, i32)>) -> Vec<i64> {
    let n = qs.len();
    let mut result = vec![0; n];
    let mut a = vec![];
    let mut sum_hero: i64 = 0;
    for i in 0..n {
        if qs[i].0 == 1 {
            sum_hero += qs[i].1 as i64;
        }
        a.push((qs[i].1, i, if qs[i].0 == 1 { 1 } else { -1 }));
        result[i] += sum_hero;
    }
    a.sort();
    a.reverse();

    let tree_size = 2 * n.next_power_of_two();
    let mut tree = vec![Node::default(); tree_size];

    for i in 0..n {
        let (x, t0, d) = a[i];
        update(&mut tree, 1, 0, n, t0, n, d, x as i64);
    }
    for i in 0..n {
        result[i] += query_result(&mut tree, 1, 0, n, i);
    }

    result
}

pub static TEST_TYPE: TestType = TestType::Single;
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

fn insert(a: &mut Vec<i32>, x: i32) {
    let (Ok(i) | Err(i)) = a.binary_search_by(|el| x.cmp(el));
    a.insert(i, x);
}

fn brute(qs: Vec<(i32, i32)>) -> Vec<i64> {
    let mut results = vec![];
    let mut heros = vec![];
    let mut artifacts = vec![];
    for q in qs {
        if q.0 == 1 {
            insert(&mut heros, q.1);
        } else {
            insert(&mut artifacts, q.1);
        }
        let mut result = 0 as i64;
        for i in 0..heros.len() {
            result += heros[i] as i64;
            if i < artifacts.len() {
                result += artifacts[i].min(heros[i]) as i64;
            }
        }
        results.push(result);
    }
    results
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(20000..=100000);
        let qs: Vec<_> = (0..n)
            .map(|_| {
                let t = rng.gen_range(1..=2);
                let v = rng.gen_range(1..=1000000000);
                (t as i32, v as i32)
            })
            .collect();

        let t0 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();

        let a = doit(qs.clone());
        let t1 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        eprintln!("{:?}", t1 - t0);
        let b = brute(qs.clone());
        let t2 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        eprintln!("{:?} {:?}", t1 - t0, t2 - t1);
        if a != b {
            eprintln!("Mismatch at {n} {qs:?}. got {a:?} vs. {b:?}");
            break;
        }
    }
}
//END MAIN
