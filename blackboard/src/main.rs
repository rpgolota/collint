fn computational_cost(m: u32, t: f64) -> f64 {
    (m as f64) * t / 3628800.0f64
}

fn main() {
    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    println!("m,b,t,c");
    for _ in 0..10 {
        handles.push(std::thread::spawn(|| {
            for m in 15..30 {
                for b in 15..30 {
                    let res = blackboard::blackboard(m, b).unwrap();
                    let c = computational_cost(res.m, res.t_star);
                    println!("{},{},{},{}", res.m, res.b, res.t_star, c);
                }
            }
        }));
    }
    for h in handles.into_iter() {
        h.join().unwrap();
    }
}
