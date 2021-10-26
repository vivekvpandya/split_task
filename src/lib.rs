use std::sync::{Arc, Mutex};
use std::thread;

pub fn split_task<T, R>(data: Vec<T>, f: fn(T) -> R) -> Vec<R>
where
    T: Clone + Copy + Send + Sync + 'static,
    R: Copy + Send + 'static,
{
    const THRESHOLD: usize = 4;
    let res = Vec::<R>::new();
    let shared = Arc::new(Mutex::new(res));

    if data.len() <= THRESHOLD {
        for d in data {
            let mut r = shared.lock().unwrap();
            r.push(f(d));
        }
    } else {
        let shared_clone = Arc::clone(&shared);
        let thread = thread::spawn(move || {
            let mut r = shared_clone.lock().unwrap();
            let rs = split_task(data[4..].to_vec(), f);
            for d in data[0..4].iter() {
                r.push(f(*d));
            }
            for t in rs {
                r.push(t);
            }
        });
        thread.join().unwrap();
    }

    Arc::try_unwrap(shared)
        .unwrap_or_else(|_| panic!("all threads have been joined"))
        .into_inner()
        .unwrap_or_else(|_| panic!("panic"))
}

#[cfg(test)]
mod tests {
    use crate::split_task;

    fn add_1(x: i32) -> i32 {
        x + 1
    }
    #[test]
    fn it_works() {
        let res = split_task(vec![1, 2, 3, 4, 5, 6, 7, 8], add_1);
        assert_eq!(res, vec![2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
