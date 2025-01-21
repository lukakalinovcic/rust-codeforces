//{"name":"E. Cells Arrangement","group":"Codeforces - Codeforces Round 943 (Div. 3)","url":"https://codeforces.com/contest/1968/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n3\n4\n5\n6\n","output":"1 1\n1 2\n\n2 1\n2 3\n3 1\n\n1 1\n1 3\n4 3\n4 4\n\n1 1\n1 3\n1 4\n2 1\n5 5\n\n1 4\n1 5\n1 6\n5 2\n5 5\n6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECellsArrangement"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let result = if n >= 4 {
        doit(n)
    } else if n == 3 {
        vec![(1, 1), (1, 3), (2, 3)]
    } else {
        vec![(1, 1), (1, 2)]
    };
    for p in result.into_iter() {
        out.print_line(p);
    }
    out.print_line("");
}

fn dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn doit(n: usize) -> Vec<(i32, i32)> {
    let mut have = vec![false; 2 * n];
    let n = n as i32;
    let mut p = vec![(1, 1), (1, n - 1), (n, n - 1), (n, n)];
    for i in 0..4 {
        for j in i..4 {
            let d = dist(p[i], p[j]);
            have[d as usize] = true;
        }
    }
    for i in 4..n as usize {
        let x = 1;
        let mut mx = 0;
        let mut how = None;
        for y in 2..n - 1 {
            let mut score = 0;
            for j in 0..4 {
                let d = dist((x, y), p[j]);
                if !have[d as usize] {
                    score += 1;
                }
            }
            if score > mx {
                mx = score;
                how = Some(y);
            }
        }
        if let Some(y) = how {
            for j in 0..4 {
                let d = dist((x, y), p[j]);
                have[d as usize] = true;
            }
            p.push((x, y));
        } else {
            for j in i..n as usize {
                p.push((2, j as i32));
            }
            break;
        }
    }
    p
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    for n in 4..=1000 {
        doit(n);
    }

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
