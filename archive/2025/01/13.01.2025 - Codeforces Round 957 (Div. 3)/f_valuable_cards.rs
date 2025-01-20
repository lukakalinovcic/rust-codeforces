//{"name":"F. Valuable Cards","group":"Codeforces - Codeforces Round 957 (Div. 3)","url":"https://codeforces.com/contest/1992/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"8\n6 4\n2 3 6 2 1 2\n9 100000\n50000 25000 12500 6250 3125 2 4 8 16\n5 2\n1 1 1 1 1\n8 6\n4 3 4 3 4 3 4 3\n7 12\n6 11 1 3 11 10 2\n10 5\n2 4 4 2 4 4 4 3 1 1\n7 8\n4 6 5 1 2 4 1\n8 27\n3 9 17 26 2 20 9 3\n","output":"3\n2\n1\n1\n2\n1\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FValuableCards"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gen(f: &[(i64, i32)], i: usize, mut divisor: i64, out: &mut Vec<i64>) {
    if i == f.len() {
        out.push(divisor);
    } else {
        for _ in 0..=f[i].1 {
            gen(f, i + 1, divisor, out);
            divisor *= f[i].0;
        }
    }
}
fn divisors(mut a: i32) -> Vec<i64> {
    let mut f = vec![];
    for d in 2.. {
        if d * d > a {
            break;
        }
        let mut cnt = 0;
        while a % d == 0 {
            cnt += 1;
            a /= d;
        }
        f.push((d as i64, cnt));
    }
    if a > 1 {
        f.push((a as i64, 1))
    }
    let mut divisors = vec![];
    gen(&f, 0, 1, &mut divisors);
    divisors.sort();
    divisors
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let x = input.read_int();
    let a = input.read_int_vec(n);
    let divs = divisors(x);
    let n_divs = divs.len();
    let mut div_index = vec![None; 200001];
    for i in 0..n_divs {
        div_index[divs[i] as usize] = Some(i);
    }
    let mut divs_graph = vec![vec![None; n_divs]; n_divs];
    for i in 0..n_divs {
        let mut k = 0;
        for j in 0..n_divs {
            while k < n_divs && divs[i] * divs[j] > divs[k] {
                k += 1;
            }
            if k < n_divs && divs[i] * divs[j] == divs[k] {
                divs_graph[i][j] = Some(k);
            }
        }
    }

    let mut n_groups = 1;
    let mut reachable = vec![false; n_divs];
    reachable[0] = true;
    for a in a {
        let j = match div_index[a as usize] {
            None => continue,
            Some(j) => j,
        };
        for i in (0..n_divs).rev() {
            if reachable[i] {
                if let Some(k) = divs_graph[i][j] {
                    reachable[k] = true;
                }
            }
        }
        if reachable[n_divs - 1] {
            n_groups = n_groups + 1;
            reachable = vec![false; n_divs];
            reachable[0] = true;
            reachable[j] = true;
        }
    }
    out.print_line(n_groups);
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
