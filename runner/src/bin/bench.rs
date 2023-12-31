use std::{env, time::Duration};
use took::{Timer, Took};

const RUNS: usize = 1;

fn main() {
    let args: Vec<_> = env::args()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("Invalid test number"))
        .collect();

    let jobs: Vec<&runner::Job> = if args.is_empty() {
        runner::jobs().iter().collect()
    } else {
        runner::jobs()
            .iter()
            .enumerate()
            .filter_map(|(idx, j)| {
                if args.contains(&((idx + 2) / 2)) {
                    Some(j)
                } else {
                    None
                }
            })
            .collect()
    };

    println!("Benchmarking {} days with {} runs...", jobs.len() / 2, RUNS);

    let times: Vec<_> = jobs
        .iter()
        .map(|(j, n, i)| {
            (
                n,
                (0..RUNS)
                    .map(|_| {
                        let took = Timer::new();
                        j(i);
                        took.took().into_std()
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect();

    times.iter().for_each(|t| Took::from_std(t.1).describe(t.0));
    // times
    //     .iter()
    //     .for_each(|t| println!("{}: {}", t.0, t.1.as_secs_f64()));
    let time_taken = times.into_iter().map(|(_, t)| t).sum();
    if time_taken < Duration::new(1, 0) {
        println!(
            "Tests took {}. Time remaining is {}",
            Took::from_std(time_taken),
            Took::from_std(Duration::new(1, 0) - time_taken)
        );
    } else {
        println!(
            "Tests took {}. Over time budget by {}",
            Took::from_std(time_taken),
            Took::from_std(time_taken - Duration::new(1, 0))
        );
    }
}
