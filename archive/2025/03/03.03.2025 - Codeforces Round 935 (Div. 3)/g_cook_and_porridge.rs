//{"name":"G. Cook and Porridge","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3 3\n2 2\n3 1\n2 3\n5 10\n10 3\n7 1\n11 3\n5 1\n6 1\n5 20\n4 2\n7 2\n8 5\n1 5\n3 1\n5 17\n1 3\n8 2\n8 3\n2 2\n1 1\n5 14\n8 2\n4 2\n1 3\n8 3\n6 4\n1 11\n4 5\n5 14\n8 2\n4 2\n1 3\n8 3\n6 4\n","output":"3\n-1\n12\n6\n6\n1\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GCookAndPorridge"}}}

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 2000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_size();
    let ks = input.read_int_pair_vec(n);
    out.print_line(doit(n, d, ks));
}

fn doit(n: usize, d: usize, ks: Vec<(i32, i32)>) -> i32 {
    let mut next = vec![0; n + 1];
    let mut prev = vec![0; n + 1];
    let mut seen = vec![false; n];
    let mut num_seen = 0;
    for i in 1..=n {
        next[i - 1] = i;
        prev[i] = i - 1;
    }
    next[n] = 0;
    prev[0] = n;
    let mut stairs: BTreeMap<i32, usize> = BTreeMap::new();
    for i in (0..n).rev() {
        let k_i = ks[i].0;
        if stairs.is_empty() || k_i > *stairs.last_key_value().unwrap().0 {
            stairs.insert(k_i, i);
        }
    }
    stairs.insert(INF, n);
    let mut reenter = vec![vec![]; d];
    for round in 0..d {
        let x = next[n];
        if x != n {
            if !seen[x] {
                seen[x] = true;
                num_seen += 1;
                if num_seen == n {
                    return round as i32 + 1;
                }
            }
            let s_x = ks[x].1;
            let reenter_time = round + s_x as usize;
            if reenter_time < d {
                reenter[reenter_time].push(x);
            }

            {
                let y = next[x];
                next[n] = y;
                prev[y] = n;
            }
            let k_x = ks[x].0;
            if let Entry::Occupied(entry) = stairs.entry(k_x) {
                if *entry.get() == x {
                    entry.remove();
                }
            }
        }

        for x in reenter[round].drain(..).rev() {
            let k_x = ks[x].0;
            let (_, &y) = stairs.range(k_x..).next().unwrap();
            next[x] = next[y];
            prev[x] = y;
            prev[next[x]] = x;
            next[prev[x]] = x;
            stairs.insert(k_x, x);
        }
    }
    -1
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

fn brute(n: usize, d: usize, ks: Vec<(i32, i32)>) -> i32 {
    let mut queue = vec![];
    for i in 0..n {
        queue.push(i);
    }
    let mut seen = vec![false; n];
    let mut num_seen = 0;
    let mut reenter = vec![vec![]; d];
    for round in 0..d {
        if !queue.is_empty() {
            let x = queue.remove(0);
            if !seen[x] {
                seen[x] = true;
                num_seen += 1;
                if num_seen == n {
                    return round as i32 + 1;
                }
            }
            let s_x = ks[x].1;
            let reenter_time = round + s_x as usize;
            if reenter_time < d {
                reenter[reenter_time].push(x);
            }
        }

        for x in reenter[round].drain(..).rev() {
            let k_x = ks[x].0;
            let mut i = queue.len();
            while i > 0 && ks[queue[i - 1]].0 < k_x {
                i -= 1;
            }
            queue.insert(i, x);
        }
    }
    -1
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 20000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(2..=10);
        let d = rng.gen_range(1..=50);
        let ks = (0..n)
            .map(|_| (rng.gen_range(1..=10), rng.gen_range(1..=10)))
            .collect::<Vec<_>>();
        let x = doit(n, d, ks.clone());
        let y = brute(n, d, ks.clone());
        if x != y {
            eprintln!("Mismatch at {n} {d} {ks:?}  {x:?} != {y:?}");
            break;
        }
    }
}
//END MAIN
