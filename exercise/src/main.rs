use std::sync::{Condvar, Mutex};
use std::{error::Error, sync::Arc, thread, time::Duration};
use std_semaphore::Semaphore;

const N: usize = 5;

fn main() -> Result<(), Box<dyn Error>> {
    // run_philosophers_problem_dead_lock()?;
    // run_philosophers_problem_easy_solution()?;
    run_philosophers_problem_taking_both_sticks_condvar()?;
    Ok(())
}

fn run_philosophers_problem_dead_lock() -> Result<(), Box<dyn Error>> {
    let mut join_handles = vec![];
    let sticks = (0..N)
        .map(|_| Arc::new(Semaphore::new(1)))
        .collect::<Vec<_>>();

    for id in 0..(N - 1) {
        let left_stick = sticks[id].clone();
        let right_stick = sticks[id + 1].clone();

        join_handles.push(thread::spawn(move || {
            run_one_left_philosopher(id, left_stick, right_stick)
        }));
    }
    join_handles.push(thread::spawn(move || {
        run_one_left_philosopher(N - 1, sticks[N - 1].clone(), sticks[0].clone())
    }));

    join_handles.into_iter().for_each(|jh| jh.join().unwrap());

    Ok(())
}

fn run_philosophers_problem_easy_solution() -> Result<(), Box<dyn Error>> {
    let mut join_handles = vec![];
    let sticks = (0..N)
        .map(|_| Arc::new(Semaphore::new(1)))
        .collect::<Vec<_>>();

    for id in 0..(N - 1) {
        let left_stick = sticks[id].clone();
        let right_stick = sticks[id + 1].clone();

        join_handles.push(thread::spawn(move || {
            run_one_left_philosopher(id, left_stick, right_stick)
        }));
    }
    join_handles.push(thread::spawn(move || {
        run_one_right_philosopher(N - 1, sticks[N - 1].clone(), sticks[0].clone())
    }));

    join_handles.into_iter().for_each(|jh| jh.join().unwrap());

    Ok(())
}

fn run_philosophers_problem_taking_both_sticks_condvar() -> Result<(), Box<dyn Error>> {
    let mut join_handles = vec![];
    let sticks = Arc::new((Mutex::new(N), Condvar::new()));

    for id in 0..N {
        let sticks = sticks.clone();
        join_handles.push(thread::spawn(move || {
            run_one_philosopher_taking_both_sticks(id, sticks)
        }));
    }
    thread::sleep(Duration::from_secs(1));
    sticks.1.notify_all();
    join_handles.into_iter().for_each(|jh| jh.join().unwrap());

    Ok(())
}

fn run_one_philosopher_taking_both_sticks(id: usize, sticks: Arc<(Mutex<usize>, Condvar)>) {
    loop {
        println!("[FILOSOFO {}] Pensando", id);
        thread::sleep(Duration::from_secs(1));

        {
            let mut sticks_left = sticks
                .1
                .wait_while(sticks.0.lock().unwrap(), |n| *n < 2)
                .unwrap();

            *sticks_left -= 2;
            println!(
                "[FILOSOFO {}] Agarro ambos palitos. Palitos restantes: {}",
                id, sticks_left
            );
        }

        println!("[FILOSOFO {}] Comiendo", id);
        thread::sleep(Duration::from_secs(1));

        *sticks.0.lock().unwrap() += 2;
        println!("[FILOSOFO {}] Suelto palitos", id);

        // sticks.1.notify_all();
        sticks.1.notify_one();
    }
}

fn run_one_left_philosopher(id: usize, left_stick: Arc<Semaphore>, right_stick: Arc<Semaphore>) {
    loop {
        println!("[FILOSOFO {}] Pensando", id);

        left_stick.acquire();

        println!("[FILOSOFO {}] Agarro palito izquierdo.", id);

        right_stick.acquire();

        println!("[FILOSOFO {}] Agarro palito derecho", id);
        println!("[FILOSOFO {}] Comiendo", id);

        thread::sleep(Duration::from_secs(1));

        left_stick.release();
        right_stick.release();

        println!("[FILOSOFO {}] Suelto palitos", id);
    }
}

fn run_one_right_philosopher(id: usize, left_stick: Arc<Semaphore>, right_stick: Arc<Semaphore>) {
    loop {
        println!("[FILOSOFO {}] Pensando", id);

        right_stick.acquire();

        println!("[FILOSOFO {}] Agarro palito derecho", id);

        left_stick.acquire();

        println!("[FILOSOFO {}] Agarro palito izquierdo.", id);
        println!("[FILOSOFO {}] Comiendo", id);

        thread::sleep(Duration::from_secs(1));

        left_stick.release();
        right_stick.release();

        println!("[FILOSOFO {}] Suelto palitos", id);
    }
}
