#![feature(test)]
extern crate test;
extern crate parking_lot;

#[cfg(test)]
mod tests {
    use test::Bencher;
    static SIZE: usize = 100000;

    #[bench]
    fn bench_mutex(bencher: &mut Bencher) {
        use std::sync::Mutex;
        let v: Vec<Mutex<usize>> = (0..SIZE).map(|i| Mutex::new(i)).collect();
        bencher.iter(|| {
            let sum = v.iter().fold(0, |acc, ref m| {
                match m.lock() {
                    Ok(guard) => acc + *guard,
                    _ => acc,
                }
            });
            sum
        });
    }

    #[bench]
    fn bench_parking_lot_mutex(bencher: &mut Bencher) {
        use parking_lot::Mutex;
        let v: Vec<Mutex<usize>> = (0..SIZE).map(|i| Mutex::new(i)).collect();
        bencher.iter(|| {
            let sum = v.iter().fold(0, |acc, ref m| {
                match m.try_lock() {
                    Some(guard) => acc + *guard,
                    _ => acc,
                }
            });
            sum
        });
    }

    #[bench]
    fn bench_parking_lot_mutex_without_branch(bencher: &mut Bencher) {
        use parking_lot::Mutex;
        let v: Vec<Mutex<usize>> = (0..SIZE).map(|i| Mutex::new(i)).collect();
        bencher.iter(|| {
            let sum = v.iter().fold(0, |acc, ref m| {
                acc + *m.lock()
            });
            sum
        });
    }

    #[bench]
    fn bench_option(bencher: &mut Bencher) {
        use std::sync::Mutex;
        let v: Vec<Option<usize>> = (0..SIZE).map(|i| Some(i)).collect();
        bencher.iter(|| {
            let sum = v.iter().fold(0, |acc, o| {
                match o {
                     &Some(val) => acc + val,
                    _ => acc,
                }
            });
            sum
        });
    }
}
