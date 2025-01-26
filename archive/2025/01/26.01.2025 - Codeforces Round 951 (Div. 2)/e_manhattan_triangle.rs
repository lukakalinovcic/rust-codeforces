//{"name":"E. Manhattan Triangle","group":"Codeforces - Codeforces Round 951 (Div. 2)","url":"https://codeforces.com/contest/1979/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n6 4\n3 1\n0 0\n0 -2\n5 -3\n3 -5\n2 -2\n5 4\n0 0\n0 -2\n5 -3\n3 -5\n2 -2\n6 6\n3 1\n0 0\n0 -2\n5 -3\n3 -5\n2 -2\n4 4\n3 0\n0 3\n-3 0\n0 -3\n10 8\n2 1\n-5 -1\n-4 -1\n-5 -3\n0 1\n-2 5\n-4 4\n-4 2\n0 0\n-4 1\n4 400000\n100000 100000\n-100000 100000\n100000 -100000\n-100000 -100000\n","output":"2 6 1\n4 3 5\n3 5 1\n0 0 0\n6 1 3\n0 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EManhattanTriangle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let d = input.read_int();
    let p = input.read_int_pair_vec(n);
    let mut result = doit(p.clone(), d);
    if result.is_none() {
        result = doit(p.into_iter().map(|(x, y)| (-x, y)).collect(), d);
    }
    let result = result.unwrap_or_default();
    out.print_line(result);
}

fn doit(p: Vec<(i32, i32)>, d: i32) -> Option<(usize, usize, usize)> {
    let n = p.len();
    let mut a = p
        .iter()
        .enumerate()
        .map(|(i, (x, y))| (x + y, *x, i))
        .collect::<Vec<_>>();
    a.sort();
    let mut b = vec![];

    let mut i = 0;
    for j in 0..n {
        while i < j && (a[j].0 != a[i].0 || 2 * (a[j].1 - a[i].1) > d) {
            i += 1;
        }
        if a[j].0 == a[i].0 && 2 * (a[j].1 - a[i].1) == d {
            b.push((a[i].0, a[i].1, (a[i].2, a[j].2)));
        }
    }

    for i in 0..n {
        let (x, y) = p[i];

        {
            let d1 = x + y + d;
            let (Ok(j) | Err(j)) = b.binary_search(&(d1, x, (0, 0)));
            if j < b.len() && b[j].0 == d1 && b[j].1 <= x + d / 2 {
                let (k, l) = b[j].2;
                return Some((i + 1, k + 1, l + 1));
            }
        }
        {
            let d2 = x + y - d;
            let (Ok(j) | Err(j)) = b.binary_search(&(d2, x - d, (0, 0)));
            if j < b.len() && b[j].0 == d2 && b[j].1 <= x - d / 2 {
                let (k, l) = b[j].2;
                return Some((i + 1, k + 1, l + 1));
            }
        }
    }

    None
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
