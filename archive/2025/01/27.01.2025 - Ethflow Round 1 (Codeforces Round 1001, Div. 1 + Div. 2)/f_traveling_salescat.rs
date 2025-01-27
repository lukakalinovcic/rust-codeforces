//{"name":"F. Traveling Salescat","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n0 2\n2 1\n3 3\n5\n2 7\n7 5\n6 3\n1 8\n7 5\n8\n899167687 609615846\n851467150 45726720\n931502759 23784096\n918190644 196992738\n142090421 475722765\n409556751 726971942\n513558832 998277529\n294328304 434714258\n","output":"4 9\n10 22 34 46\n770051069 1655330585 2931719265 3918741472 5033924854 6425541981 7934325514\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTravelingSalescat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn update(x: &mut i64, y: i64) {
    if y < *x {
        *x = y;
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut ab = input.read_long_pair_vec(n);
    ab.sort_by_key(|(a, b)| b - a);
    let mut curr = vec![[[INF, INF, INF]; 5]; n + 1];
    curr[0][0][0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        let mut next = curr.clone();
        for k in 0..=i {
            for crossing in 0..=(2 * k).min(2) {
                for free_ends in 0..=2 {
                    let c = curr[k][crossing][free_ends];
                    if crossing == 0 && k > 0 {
                        continue;
                    }

                    // new free end, looking down.
                    if free_ends < 2 {
                        update(&mut next[k + 1][crossing + 1][free_ends + 1], c + a);
                    }

                    // new free end, looking up.
                    if crossing >= 1 && free_ends < 2 {
                        update(&mut next[k + 1][crossing - 1][free_ends + 1], c + b);
                    }

                    // run through.
                    if crossing >= 1 {
                        update(&mut next[k + 1][crossing][free_ends], c + a + b);
                    }

                    // bottom turn.
                    if crossing >= 2 {
                        update(&mut next[k + 1][crossing - 2][free_ends], c + 2 * b);
                    }

                    // top turn.
                    update(&mut next[k + 1][crossing + 2][free_ends], c + 2 * a);
                }
            }
        }
        curr = next;
    }
    let mut results = vec![];
    for k in 2..=n {
        results.push(curr[k][0][2]);
    }
    out.print_line(results);
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
