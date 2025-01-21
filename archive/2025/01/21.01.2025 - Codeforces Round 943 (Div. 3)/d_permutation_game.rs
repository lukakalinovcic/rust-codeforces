//{"name":"D. Permutation Game","group":"Codeforces - Codeforces Round 943 (Div. 3)","url":"https://codeforces.com/contest/1968/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n4 2 3 2\n4 1 2 3\n7 2 5 6\n10 8 2 10\n3 1 4 5 2 7 8 10 6 9\n5 10 5 1 3 7 10 15 4 3\n2 1000000000 1 2\n1 2\n4 4\n8 10 4 1\n5 1 4 3 2 8 6 7\n1 1 2 1 2 100 101 102\n5 1 2 5\n1 2 4 5 3\n4 6 9 4 2\n4 2 3 1\n4 1 3 2\n6 8 5 3\n6 9 5 4\n6 1 3 5 2 4\n6 9 8 9 5 10\n4 8 4 2\n2 3 4 1\n5 2 8 7\n4 2 3 1\n4 1 3 2\n6 8 5 3\n2 1000000000 1 2\n1 2\n1000000000 2\n","output":"Bodya\nSasha\nDraw\nDraw\nBodya\nSasha\nSasha\nSasha\nSasha\nBodya\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPermutationGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let k = input.read_int();
    let x = input.read_int() - 1;
    let y = input.read_int() - 1;
    let p = input
        .read_int_vec(n)
        .into_iter()
        .map(|x| x - 1)
        .collect::<Vec<_>>();
    let a = input.read_int_vec(n);

    let xscore = doit(&p, &a, k, x);
    let yscore = doit(&p, &a, k, y);
    if xscore > yscore {
        out.print_line("Bodya");
    } else if xscore < yscore {
        out.print_line("Sasha");
    } else {
        out.print_line("Draw");
    }
}

fn doit(p: &[i32], a: &[i32], k: i32, x: i32) -> i64 {
    let n = p.len();
    let mut first = vec![(0, 0); n];
    let mut result = 0;
    let mut x = x;
    let mut sum: i64 = 0;
    for i in 1..=k {
        sum += a[x as usize] as i64;
        if first[x as usize].0 == 0 {
            first[x as usize] = (i, sum);
        } else {
            break;
        }
        x = p[x as usize];
    }
    for x in 0..n {
        let (t, sum) = first[x as usize];
        if t > 0 {
            result = result.max(sum + (k - t) as i64 * a[x as usize] as i64);
        }
    }
    result
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
