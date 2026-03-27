// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // lock the mutex, then update the shared value
            let mut s = status_shared.lock().unwrap();
            s.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // lock and read the current completed count
        let result = status.lock().unwrap();
        println!("jobs completed {}", result.jobs_completed);
    }
}
/*
功能：在 Mutex<T> 上调用 lock() 会阻塞直到获得互斥锁，返回 Result<MutexGuard<T>, PoisonError>（成功时是 Ok(MutexGuard)）。
MutexGuard：是一个智能指针，Deref 到内部 T，在 MutexGuard 被丢弃（超出作用域或显式 drop）时自动释放锁。
.unwrap()：常见写法 let mut g = m.lock().unwrap(); ——它会在锁“中毒”（poisoned，某个持锁线程 panic 导致）时 panic。如果想优雅处理，用 match 或 map_err。
例：
match m.lock() { Ok(mut g) => g.x += 1, Err(poison) => /* 处理或恢复 */ }
中毒（poisoning）：当持锁线程 panic，标准库把锁标记为中毒，后续 lock() 返回 Err(PoisonError)，但仍可通过 into_inner() 或 into_inner().unwrap() 等取出数据并决定如何处理。
非阻塞尝试：try_lock() 返回 Option<MutexGuard>（或 Result，取决于实现），用于不想阻塞的场景。标准库提供 try_lock()。
没有超时：标准 std::sync::Mutex 没有超时机制；若需要超时或更灵活的锁，请用 parking_lot 等第三方库。
作用域控制（重要）：尽量把持锁的代码限制在小块内，避免长时间持锁（会影响并发并易死锁）。用块或显式 drop 缩短锁生命周期：
示例：
或读后马上释放：
在你的 threads2.rs 中的用法：status.lock().unwrap() 返回一个 MutexGuard<JobStatus>，你可以通过 guard.jobs_completed 访问/修改字段；当 guard 离开作用域时锁自动释放，所以 println!("jobs completed {}", result.jobs_completed); 在 result 存活期间仍持锁（可把读取包装到小块里以尽快释放锁）。
需要我把 threads2.rs 的打印处改成使用短作用域读取并解释输出顺序吗？- 在你的 threads2.rs 中的用法：status.lock().unwrap() 返回一个 MutexGuard<JobStatus>，你可以通过 guard.jobs_completed 访问/修改字段；当 guard 离开作用域时锁自动释放，所以 println!("jobs completed {}", result.jobs_completed); 在 result 存活期间仍持锁（可把读取包装到小块里以尽快释放锁）。
*/