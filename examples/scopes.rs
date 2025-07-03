use core::hash::Hasher;
use core::time::Duration;
use std::hash::RandomState;
use std::sync::Mutex;

use job_pool::{PoolConfig, ThreadPool};

pub fn main() {
    let conf = PoolConfig::builder().max_jobs(16).build();
    let pool = ThreadPool::new(conf).unwrap();

    let nums = (0..1000).collect::<Vec<_>>();

    let n = Mutex::new(0);

    fn delay() {
        use core::hash::BuildHasher;
        let rand = RandomState::new().build_hasher().finish();
        let millis = rand % 500;
        std::thread::sleep(Duration::from_millis(1000 + millis));
    }

    pool.scope(|scope| {
        scope.subscope(|sc| {
            sc.execute(|| {
                delay();
                *n.lock().unwrap() += nums.iter().sum::<usize>();
                println!("Sum1");
            });
            sc.execute(|| {
                delay();
                *n.lock().unwrap() += nums.iter().filter(|n| *n % 2 == 0).sum::<usize>();
                println!("Sum even");
            });
        });

        scope.subscope(|sc| {
            sc.execute(|| {
                delay();
                *n.lock().unwrap() *= nums.iter().max().unwrap();
                println!("Mul max");
            });

            sc.execute(|| {
                delay();
                *n.lock().unwrap() *= nums[nums.len() / 2];
                println!("Mul mid");
            });
        });
    });

    let mut expected = 0;
    expected += nums.iter().sum::<usize>();
    expected += nums.iter().filter(|n| *n % 2 == 0).sum::<usize>();
    expected *= nums.iter().max().unwrap();
    expected *= nums[nums.len() / 2];

    let n = *n.lock().unwrap();
    assert_eq!(n, expected);
    println!("{n} == {expected}");
}
