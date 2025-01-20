//{"name":"D. Refined Product Optimality","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 4\n1 1 2\n3 2 1\n1 3\n2 3\n1 1\n2 1\n6 8\n1 4 2 7 3 5\n7 6 5 6 3 3\n2 5\n1 6\n1 5\n1 5\n1 5\n2 3\n2 3\n1 6\n13 8\n7 7 6 6 5 5 5 2 2 3 4 5 1\n1 4 1 9 6 6 9 1 5 1 3 8 4\n2 2\n2 11\n2 4\n2 4\n1 7\n1 1\n2 12\n1 5\n5 3\n10000000 20000000 30000000 40000000 50000000\n10000000 20000000 30000000 40000000 50000000\n1 1\n2 2\n2 1\n","output":"2 3 3 6 6\n840 840 1008 1344 1680 2016 2016 2016 2352\n2116800 2646000 3528000 3528000 3528000 4233600 4838400 4838400 4838400\n205272023 205272023 205272023 264129429\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRefinedProductOptimality"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i64 = 998244353;

struct SortedSeq {
    value: Vec<i32>,
    order: Vec<i32>,
    position: Vec<i32>,
}

impl SortedSeq {
    fn new(a: Vec<i32>) -> Self {
        let value = a.to_vec();
        let mut order: Vec<i32> = (0..a.len()).map(|x| x as i32).collect();
        order.sort_by(|i, j| {
            let x = value[(*i) as usize];
            let y = value[(*j) as usize];
            x.cmp(&y)
        });
        let mut position = vec![0; a.len()];
        for (i, o) in order.iter().enumerate() {
            position[(*o) as usize] = i as i32;
        }
        Self {
            value,
            order,
            position,
        }
    }

    fn push_right(&mut self, i: usize) {
        let v = self.value[i];
        let p = self.position[i] as usize;
        let mut lo = p;
        let mut hi = self.value.len();
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.value[self.order[mid] as usize] > v {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        let j = self.order[lo] as usize;
        self.order.swap(p, lo);
        self.position[i] = lo as i32;
        self.position[j] = p as i32;
    }

    fn inc(&mut self, i: usize) {
        self.value[i] += 1;
    }

    fn pos(&self, i: usize) -> usize {
        self.position[i] as usize
    }

    fn get(&self, p: usize) -> i32 {
        self.value[self.order[p] as usize]
    }
}

fn pow(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 1 {
        (a * pow(a, n - 1)) % MOD
    } else {
        let x = pow(a, n / 2);
        (x * x) % MOD
    }
}

fn inverse(a: i64) -> i64 {
    pow(a, MOD - 2)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, q) = (input.read_int() as usize, input.read_int() as usize);
    let mut a = SortedSeq::new((0..n).map(|_| input.read_int()).collect());
    let mut b = SortedSeq::new((0..n).map(|_| input.read_int()).collect());
    let mut results = vec![];
    let mut result = 1;
    for i in 0..n {
        let aa = a.get(i) as i64;
        let bb = b.get(i) as i64;
        result = (result * aa.min(bb)) % MOD;
    }
    results.push(result);
    for _ in 0..q {
        let (o, i) = (input.read_int() as usize, input.read_int() as usize - 1);
        let p = if o == 1 {
            a.push_right(i);
            a.pos(i)
        } else {
            b.push_right(i);
            b.pos(i)
        };
        {
            let aa = a.get(p) as i64;
            let bb = b.get(p) as i64;
            result = (result * inverse(aa.min(bb))) % MOD;
        }
        if o == 1 {
            a.inc(i);
        } else {
            b.inc(i);
        };
        {
            let aa = a.get(p) as i64;
            let bb = b.get(p) as i64;
            result = (result * aa.min(bb)) % MOD;
        }
        results.push(result);
    }
    out.print_iter(results.into_iter());
    out.put(b'\n');
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
