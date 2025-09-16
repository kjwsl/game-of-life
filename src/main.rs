use game_of_life::Canvas;

fn main() {
    let ms = 300;
    let canvas_path = std::env::args().nth(1).expect("Specify canvas path");
    let canvas_file = std::fs::read_to_string(canvas_path).expect("Failed to read canvas path");

    let mut canvas = Canvas::from_str(&canvas_file);

    loop {
        print!("\x1B[2J\x1B[H");
        canvas.print();
        canvas.tick();
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
