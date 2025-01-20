//{"name":"D. I Love 1543","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n2 4\n1543\n7777\n2 4\n7154\n8903\n2 4\n3451\n8888\n2 2\n54\n13\n2 2\n51\n43\n2 6\n432015\n512034\n4 4\n5431\n1435\n5518\n7634\n6 4\n5432\n1152\n4542\n2432\n2302\n5942\n","output":"1\n1\n0\n1\n0\n2\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DILove1543"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let data: Vec<_> = (0..n).map(|_| input.read_line().into_bytes()).collect();
    let mut result = 0;
    for i in 0..(n / 2).min(m / 2) {
        let mut layer = vec![];
        for c in i..m - i {
            layer.push(data[i][c]);
        }
        for r in i + 1..n - i - 1 {
            layer.push(data[r][m - i - 1]);
        }
        for c in (i..m - i).rev() {
            layer.push(data[n - i - 1][c]);
        }
        for r in (i + 1..n - i - 1).rev() {
            layer.push(data[r][i]);
        }
        for i in 0..3 {
            layer.push(layer[i]);
        }
        for i in 0..layer.len() - 3 {
            if layer[i] == b'1'
                && layer[i + 1] == b'5'
                && layer[i + 2] == b'4'
                && layer[i + 3] == b'3'
            {
                result += 1;
            }
        }
    }
    out.print_line(result);
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
