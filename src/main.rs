use clap::Parser;
use game_of_life::Canvas;

const DEFAULT_INTERVAL: u64 = 300;

#[derive(Parser)]
struct Args {
    canvas_path: String,

    /// The number of milliseconds to wait between each tick
    #[clap(short, long, default_value_t = DEFAULT_INTERVAL)]
    interval: u64,

    /// The number of generations to simulate
    #[clap(long, default_value_t = false)]
    infinite: bool,
}

fn main() {
    let args = Args::parse();

    let ms = args.interval;
    let canvas_file = std::fs::read_to_string(args.canvas_path).expect("Failed to read canvas path");

    let mut canvas = Canvas::from_str(&canvas_file);
    canvas.set_infinite(args.infinite);

    loop {
        print!("\x1B[2J\x1B[H");
        canvas.print();
        canvas.tick();
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
