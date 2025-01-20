//{"name":"E. Adventurers","group":"Codeforces - Codeforces Round 990 (Div. 2)","url":"https://codeforces.com/contest/2047/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n4\n1 1\n1 2\n2 1\n2 2\n4\n0 0\n0 0\n0 0\n0 0\n8\n1 2\n2 1\n2 -1\n1 -2\n-1 -2\n-2 -1\n-2 1\n-1 2\n7\n1 1\n1 2\n1 3\n1 4\n2 1\n3 1\n4 1\n","output":"1\n2 2\n0\n0 0\n2\n1 0\n0\n0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAdventurers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn add(tree: &mut [i32], i: usize, lo: usize, hi: usize, pos: usize, delta: i32) {
    if pos < lo || pos >= hi {
        return;
    }
    tree[i] += delta;
    if hi - lo > 1 {
        let mid = (lo + hi) / 2;
        add(tree, 2 * i, lo, mid, pos, delta);
        add(tree, 2 * i + 1, mid, hi, pos, delta);
    }
}

fn query(tree: &[i32], i: usize, lo: usize, hi: usize, k: i32) -> usize {
    if hi - lo == 1 {
        lo
    } else {
        let mid = (lo + hi) / 2;
        if tree[2 * i] >= k {
            query(tree, 2 * i, lo, mid, k)
        } else {
            query(tree, 2 * i + 1, mid, hi, k - tree[2 * i])
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut xy_pts = vec![];
    let mut x_order = vec![];
    let mut y_order = vec![];
    for (i, (x, y)) in input.read_int_pair_vec(n).into_iter().enumerate() {
        x_order.push(i);
        y_order.push(i);
        xy_pts.push((x, y));
    }
    x_order.sort_by(|i, j| xy_pts[*i as usize].0.cmp(&xy_pts[*j as usize].0));
    y_order.sort_by(|i, j| xy_pts[*i as usize].1.cmp(&xy_pts[*j as usize].1));
    let mut ij_pts = vec![(0, 0); n];
    for (pos, i) in x_order.iter().enumerate() {
        ij_pts[*i].0 = pos;
    }
    for (pos, j) in y_order.iter().enumerate() {
        ij_pts[*j].1 = pos;
    }

    let mut m = 1;
    while m < 2 * n {
        m *= 2;
    }
    let mut left_tree = vec![0; m];
    let mut right_tree = vec![0; m];
    for i in x_order.iter() {
        add(&mut left_tree, 1, 0, n, ij_pts[*i].1, 1);
    }
    let mut k = 0;
    let mut best = (0, 0);
    for (curr, next) in x_order.iter().rev().zip(x_order.iter().rev().skip(1)) {
        add(&mut left_tree, 1, 0, n, ij_pts[*curr].1, -1);
        add(&mut right_tree, 1, 0, n, ij_pts[*curr].1, 1);

        if xy_pts[*curr].0 != xy_pts[*next].0 {
            loop {
                let left_cnt = left_tree[1];
                let right_cnt = right_tree[1];
                if left_cnt < 2 * (k + 1) || right_cnt < 2 * (k + 1) {
                    break;
                }
                let left_lo = query(&left_tree, 1, 0, n, k + 1);
                let left_hi = query(&left_tree, 1, 0, n, left_cnt - k);
                let left_lo_y = xy_pts[y_order[left_lo]].1;
                let left_hi_y = xy_pts[y_order[left_hi]].1;

                let right_lo = query(&right_tree, 1, 0, n, k + 1);
                let right_hi = query(&right_tree, 1, 0, n, right_cnt - k);
                let right_lo_y = xy_pts[y_order[right_lo]].1;
                let right_hi_y = xy_pts[y_order[right_hi]].1;

                let lo_y = left_lo_y.max(right_lo_y);
                let hi_y = left_hi_y.min(right_hi_y);
                if lo_y >= hi_y {
                    break;
                }
                let x = xy_pts[*curr].0;
                k += 1;
                best = (x, hi_y);
            }
        }
    }
    out.print_line(k);
    out.print_line(best);
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
