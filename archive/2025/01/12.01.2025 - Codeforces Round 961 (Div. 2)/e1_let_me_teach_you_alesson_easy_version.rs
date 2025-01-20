//{"name":"E1. Let Me Teach You a Lesson (Easy Version)","group":"Codeforces - Codeforces Round 961 (Div. 2)","url":"https://codeforces.com/contest/1995/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n6 6 4 4\n1\n10 17\n3\n1 10 1 10 1 10\n3\n3 3 4 5 5 4\n5\n1 2 3 4 5 6 7 8 9 10\n","output":"0\n0\n0\n2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1LetMeTeachYouALessonEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

type TwoByTwo = [[bool; 2]; 2];

const INF: i32 = 2000000000;

fn solve_even(n: usize, a: Vec<i32>) -> i32 {
    let mut global_min = INF;
    let mut global_max = 0;
    for i in (0..n).step_by(2) {
        let (x1, x2) = (a[i], a[i + 1]);
        let (y1, y2) = (a[i + n], a[i + 1 + n]);

        let mut min = 0;
        let mut max = INF;
        for swap1 in [true, false] {
            for swap2 in [true, false] {
                let (x1, y1) = if swap1 { (y1, x1) } else { (x1, y1) };
                let (x2, y2) = if swap2 { (y2, x2) } else { (x2, y2) };
                let x = x1 + x2;
                let y = y1 + y2;
                min = min.max(x.min(y));
                max = max.min(x.max(y));
            }
        }
        global_min = global_min.min(min);
        global_max = global_max.max(max);
    }
    global_max - global_min
}

fn gen_odd_order(n: usize) -> Vec<usize> {
    let mut x = 0;
    let mut order = vec![];
    for i in 0..2 * n {
        order.push(x);
        if i % 2 == 0 {
            x ^= 1;
        } else if x < n {
            x += n;
        } else {
            x -= n;
        }
    }
    order
}

fn merge(a: &TwoByTwo, b: &TwoByTwo) -> TwoByTwo {
    let mut c = [[false; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                c[i][j] |= a[i][k] & b[k][j];
            }
        }
    }
    c
}

fn tree_update(tree: &mut Vec<TwoByTwo>, mut i: usize) {
    while i != 1 {
        i /= 2;
        tree[i] = merge(&tree[2 * i], &tree[2 * i + 1]);
    }
}

fn solve_odd(n: usize, a: Vec<i32>) -> i32 {
    if n == 1 {
        return 0;
    }
    let order = gen_odd_order(n);
    let mut events = vec![];
    for i in 0..n {
        let (curr1, curr2) = (a[order[2 * i]], a[order[2 * i + 1]]);
        let prev = a[order[(2 * i + 2 * n - 1) % (2 * n)]];
        let next = a[order[(2 * i + 2) % (2 * n)]];
        for swap_prev in 0..2 {
            for swap_next in 0..2 {
                let a = if swap_prev == 0 { curr1 } else { prev };
                let b = if swap_next == 0 { curr2 } else { next };
                events.push((a + b, i, swap_prev, swap_next));
            }
        }
    }
    events.sort();
    let tree_shift = n.next_power_of_two();
    let tree_size = 2 * tree_shift;
    let mut tree = vec![[[false; 2]; 2]; tree_size];
    for i in n..tree_shift {
        tree[tree_shift + i][0][0] = true;
        tree[tree_shift + i][1][1] = true;
        tree_update(&mut tree, tree_shift + i);
    }

    let mut tail = 0;
    let mut result = INF;
    for head in 0..events.len() {
        let (_, i, prev, next) = events[head];
        let i = tree_shift + i;
        tree[i][prev][next] = true;
        tree_update(&mut tree, i);
        while tree[1][0][0] | tree[1][1][1] {
            result = result.min(events[head].0 - events[tail].0);
            let (_, i, prev, next) = events[tail];
            tail += 1;
            let i = tree_shift + i;
            tree[i][prev][next] = false;
            tree_update(&mut tree, i);
        }
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let a = input.read_int_vec(2 * n);
    if n % 2 == 0 {
        out.print_line(solve_even(n, a));
    } else {
        out.print_line(solve_odd(n, a));
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
