use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
    vec,
};

const N: usize = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let resourses = Arc::new((Mutex::new(vec![true, true, false]), Condvar::new()));
    let mut join_handles = vec![];

    let supermarket_res = resourses.clone();

    for id in 0..N {
        let resources = resourses.clone();
        join_handles.push(thread::spawn(move || handle_smoker(id, resources)));
    }

    join_handles.push(thread::spawn(move || handle_supermarket(supermarket_res)));
    join_handles.into_iter().for_each(|jh| jh.join().unwrap());

    Ok(())
}

fn handle_supermarket(resources: Arc<(Mutex<Vec<bool>>, Condvar)>) {
    loop {
        {
            let mut res = resources
                .1
                .wait_while(resources.0.lock().unwrap(), |res| res.iter().any(|v| *v))
                .unwrap();

            let rand1: usize = rand::random::<usize>() % 3;
            let mut rand2: usize = rand::random::<usize>() % 3;
            while rand2 == rand1 {
                rand2 = rand::random::<usize>() % 3;
            }

            println!("[SUPERMARKET] Agregando recurso {} y {}.", rand1, rand2);
            res[rand1] = true;
            res[rand2] = true;
        }
        resources.1.notify_all();
    }
}

fn handle_smoker(id: usize, resources: Arc<(Mutex<Vec<bool>>, Condvar)>) {
    loop {
        {
            let mut res = resources
                .1
                .wait_while(resources.0.lock().unwrap(), |res| {
                    let mut partial_res = vec![false; N];
                    for i in 0..N {
                        partial_res[i] = res[i] == true || id == i;
                    }
                    !partial_res.iter().all(|v| *v)
                })
                .unwrap();

            println!("[SMOKER {}] Fumando.", id);
            thread::sleep(Duration::from_secs(1));
            *res = vec![false; N];
        }
        resources.1.notify_all();
    }
}
