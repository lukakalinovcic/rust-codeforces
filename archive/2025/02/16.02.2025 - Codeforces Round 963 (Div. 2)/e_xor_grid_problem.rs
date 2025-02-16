//{"name":"E. Xor-Grid Problem","group":"Codeforces - Codeforces Round 963 (Div. 2)","url":"https://codeforces.com/contest/1993/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n1 2\n1 3\n2 3\n0 1 0\n5 4 4\n2 3\n0 2 4\n4 5 1\n3 3\n1 2 3\n4 5 6\n7 8 9\n","output":"1\n3\n13\n24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EXorGridProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut a = (0..n).map(|_| input.read_int_vec(m)).collect::<Vec<_>>();
    for r in 0..n {
        let mut x = 0;
        for c in 0..m {
            x ^= a[r][c];
        }
        a[r].push(x);
    }
    a.push(vec![]);
    for c in 0..=m {
        let mut x = 0;
        for r in 0..n {
            x ^= a[r][c];
        }
        a[n].push(x);
    }
    let (n, m) = (n + 1, m + 1);
    let c1 = doit(n, m, &a);
    let b = transpose(n, m, &a);
    let c2 = doit(m, n, &b);
    let mut result = INF;
    for r in 0..n {
        for c in 0..m {
            result = result.min(c1[r][c] + c2[c][r]);
        }
    }
    out.print_line(result);
}

fn transpose(n: usize, m: usize, a: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut b = vec![vec![0; n]; m];
    for r in 0..n {
        for c in 0..m {
            b[c][r] = a[r][c];
        }
    }
    b
}

fn doit(n: usize, m: usize, a: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![INF; m]; n];
    for r_omit in 0..n {
        let mut g = vec![vec![0; m]; m];
        for i in 0..m {
            for j in 0..m {
                for r in 0..n {
                    if r != r_omit {
                        g[i][j] += (a[r][i] - a[r][j]).abs();
                    }
                }
            }
        }
        let dp = hamiltonian_path(m, &g);
        for c_omit in 0..m {
            let mask = ((1 << m) - 1) ^ (1 << c_omit);
            for j in 0..m {
                result[r_omit][c_omit] = result[r_omit][c_omit].min(dp[mask][j]);
            }
        }
    }
    result
}

fn hamiltonian_path(n: usize, g: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut dp = vec![vec![INF; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for mask in 0..1 << n {
        for i in 0..n {
            if (mask >> i) & 1 == 0 {
                continue;
            }
            for j in 0..n {
                if (mask >> j) & 1 == 1 {
                    continue;
                }
                dp[mask | (1 << j)][j] = dp[mask | (1 << j)][j].min(dp[mask][i] + g[i][j]);
            }
        }
    }
    dp
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
