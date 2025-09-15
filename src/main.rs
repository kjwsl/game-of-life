fn main() {
    let ms = std::env::args().nth(1).unwrap_or("1000".to_string()).parse().unwrap();
    let mut canvas = game_of_life::Canvas::new(100, 20);
    canvas.set_cell(25, 4, game_of_life::Cell::Alive);
    canvas.set_cell(25, 5, game_of_life::Cell::Alive);
    canvas.set_cell(25, 6, game_of_life::Cell::Alive);
    canvas.set_cell(24, 5, game_of_life::Cell::Alive);
    canvas.set_cell(26, 6, game_of_life::Cell::Alive);

    loop {
        print!("\x1B[2J\x1B[H");
        canvas.print();
        canvas.tick();
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
