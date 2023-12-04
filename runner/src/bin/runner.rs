use gag::Gag;
use took::Timer;

fn main() {
    let timer = Timer::new();
    let print_gag = Gag::stdout().unwrap();
    runner::jobs().into_iter().for_each(|(j, _, i)| j(i));
    drop(print_gag);
    timer.took().describe("everything");
}
