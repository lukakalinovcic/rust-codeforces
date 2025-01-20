//{"name":"E. Range Minimum Sum","group":"Codeforces - Codeforces Round 958 (Div. 2)","url":"https://codeforces.com/contest/1988/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n1\n1\n3\n3 1 2\n5\n4 2 1 5 3\n8\n8 1 4 6 7 3 5 2\n","output":"0\n4 7 5\n19 21 27 17 19\n79 100 72 68 67 80 73 80\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERangeMinimumSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

#[derive(Default, Clone, Debug)]
struct Node {
    sum_x_times_y: i64,
    sum_x: i64,
    y: i64, // undefined on internal nodes.
}
type SetY = i64;

struct MySpec {}
impl LazySegTreeSpec<Node, SetY> for &MySpec {
    fn identity(&self) -> Node {
        Node {
            sum_x_times_y: 0,
            sum_x: 0,
            y: 0,
        }
    }

    fn op(&self, a: &Node, b: &Node) -> Node {
        Node {
            sum_x_times_y: a.sum_x_times_y + b.sum_x_times_y,
            sum_x: a.sum_x + b.sum_x,
            y: 0,
        }
    }

    fn compose(&self, _old: &SetY, new: &SetY) -> SetY {
        *new
    }

    fn update(&self, t: &Node, y: &SetY) -> Node {
        Node {
            sum_x_times_y: t.sum_x * y,
            sum_x: t.sum_x,
            y: *y,
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let p = input.read_long_vec(n);
    let n = p.len();
    let mut prev = vec![0; n];
    let mut l_stack = vec![(0, -1)];
    for i in 0..n {
        while p[i] < l_stack.last().unwrap().0 {
            l_stack.pop();
        }
        prev[i] = l_stack.last().unwrap().1;
        l_stack.push((p[i], i as i64));
    }
    let mut next = vec![0; n];
    let mut undo = vec![vec![]; n];
    let mut r_stack = vec![(0, n as i64)];
    let mut sum = 0;
    for i in (0..n).rev() {
        while p[i] < r_stack.last().unwrap().0 {
            let (a, j) = r_stack.pop().unwrap();
            sum += a * (j - prev[j as usize]) * (next[j as usize] - j);
            undo[i].push((a, j));
        }
        next[i] = r_stack.last().unwrap().1;
        r_stack.push((p[i], i as i64));
    }
    let tree_spec = MySpec {};
    let mut l_tree = LazySegTree::new(&tree_spec, n + 1);
    let mut r_tree = LazySegTree::new(&tree_spec, n + 1);
    l_tree.update(0, n, n as i64 - 1);
    r_tree.update(0, n, 0);
    for (a, j) in r_stack[1..].iter() {
        let (a, j) = (*a, *j);
        let x = a * (next[j as usize] - j);
        sum += j * x;
        let y = r_tree.get(a as usize).y;
        r_tree.set(
            a as usize,
            Node {
                sum_x_times_y: x * y,
                sum_x: x,
                y,
            },
        );
        l_tree.update(a as usize, n + 1, j - 1);
    }

    let mut results: Vec<i64> = vec![0; n];
    let mut l_stack = vec![(0, -1)];
    for i in 0..n {
        // Removing p[i] from the right stack.
        {
            let (a, j) = r_stack.pop().unwrap();
            debug_assert!(a == p[i]);
            debug_assert!(j == i as i64);

            let x = a * (next[j as usize] - j);
            sum -= j * x;
            r_tree.set(
                a as usize,
                Node {
                    sum_x_times_y: 0,
                    sum_x: 0,
                    y: 0,
                },
            );
        }
        {
            let (a, j) = *r_stack.last().unwrap();
            l_tree.update(a as usize, n + 1, j - 1);
        }

        // Updating the right stack.
        for (a, j) in undo[i].iter().rev() {
            let (a, j) = (*a, *j);
            let x = a * (next[j as usize] - j);
            sum -= (j - prev[j as usize]) * x;
            sum += j * x;
            let y = r_tree.get(a as usize).y;
            r_tree.set(
                a as usize,
                Node {
                    sum_x_times_y: x * y,
                    sum_x: x,
                    y,
                },
            );
            l_tree.update(a as usize, n + 1, j - 1);
            r_stack.push((a, j));
        }

        // Calculating the result.
        results[i] = sum + l_tree.all_product().sum_x_times_y - r_tree.all_product().sum_x_times_y;

        // Updating the left stack.
        while p[i] < l_stack.last().unwrap().0 {
            let (a, j) = l_stack.pop().unwrap();
            let x = a * (j - prev[j as usize]);
            sum += (next[j as usize] - j) * x;
            sum -= -j * x;
            l_tree.set(
                a as usize,
                Node {
                    sum_x_times_y: 0,
                    sum_x: 0,
                    y: 0,
                },
            );
        }

        {
            let (a, j) = (p[i], i as i64);
            let x = a * (j - prev[i]);
            sum += -j * x;
            let y = l_tree.get(a as usize).y;
            l_tree.set(
                a as usize,
                Node {
                    sum_x_times_y: x * y,
                    sum_x: x,
                    y,
                },
            );
            r_tree.update(a as usize, n + 1, j + 1);
            l_stack.push((a, j));
        }
    }

    out.print_iter(results.into_iter());
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
