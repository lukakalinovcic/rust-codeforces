//{"name":"E2. Subtangle Game (Hard Version)","group":"Codeforces - Codeforces Round 972 (Div. 2)","url":"https://codeforces.com/contest/2005/problem/E2","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2 3\n1 2\n1 3 6\n4 6 2\n2 2 4\n1 2\n1 1 3 2\n4 2 5 1\n2 4 2\n1 2\n3 4\n5 6\n7 8\n8 8\n","output":"N\nT\nN\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2SubtangleGameHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l: usize = input.read();
    let n: usize = input.read();
    let m: usize = input.read();
    let a = input.read_int_vec(l);
    let b = (0..n).map(|_| input.read_int_vec(m)).collect::<Vec<_>>();
    if doit(a, b) {
        out.print_line("T");
    } else {
        out.print_line("N");
    }
}

fn doit(a: Vec<i32>, b: Vec<Vec<i32>>) -> bool {
    let l = a.len();
    let n = b.len();
    let m = b[0].len();
    let mut positions = vec![vec![]; n * m + 1];
    for i in 0..n {
        for j in 0..m {
            positions[b[i][j] as usize].push((i, j));
        }
    }
    let mut prev_stairs = vec![-1; n + 1];
    for k in (0..l).rev() {
        let mut curr_stairs = vec![-1; n + 1];
        let pos = &positions[a[k] as usize];
        let mut i = pos.len();

        for r in (0..n).rev() {
            curr_stairs[r] = curr_stairs[r + 1];

            let (Ok(j) | Err(j)) = pos[..i].binary_search(&(r, 0));
            if j < i && pos[i - 1].1 as i32 >= prev_stairs[r + 1] {
                curr_stairs[r] = curr_stairs[r].max(pos[i - 1].1 as i32);
            }
            i = j;
        }
        prev_stairs = curr_stairs;
    }
    prev_stairs[0] != -1
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

    let l = 1500;
    let n = 1500;
    let m = 1500;
    let a = vec![1; l];
    let b = (0..n).map(|_| vec![1; m]).collect::<Vec<_>>();

    let t0 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    doit(a, b);
    let t1 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    eprintln!("{:?}", t1 - t0);
}
//END MAIN
