use rsautogui::mouse;
// use rsautogui::keyboard;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use hotkey;
use screenshots::Screen;
use std::{fs, time::Instant};

fn print_pixel_at_mouse() {
    // get mouse position
    let (mouse_x, mouse_y) = mouse::position();
    println!("Mouse is at ({}, {})", mouse_x, mouse_y);
}

fn wiggle_mouse() {
    // print the current mouse position
    let (mouse_x, mouse_y) = mouse::position();
    println!("Mouse is at ({}, {})", mouse_x, mouse_y);
    // print the color of the pixel under the mouse
    let mut random_generator = rand::thread_rng();
    let mut x:u16;
    let mut y:u16;
    let start_time = std::time::Instant::now();
    const ITERATIONS:u16 = 100;
    for _i in 0..100 {
        x = random_generator.gen_range(0..ITERATIONS);
        y = rand::thread_rng().gen_range(0..ITERATIONS);
        mouse::move_to(x, y);
        sleep(Duration::from_millis(1));
    }
    // get a precise elapsed time
    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Wiggled mouse for {} seconds", elapsed_time);
    // convert ITERATIONS to f64
    let iterations_as_float:f64 = ITERATIONS as f64;
    println!("That's {} wiggles per second", iterations_as_float/elapsed_time);
}



fn take_screenshot() {
  let start = Instant::now();
  // grab the first screen
  let screen = Screen::all().unwrap()[0];
  // if there's no folder called screenshots, create it
  fs::create_dir_all("screenshots").unwrap();
  println!("capturer {:?}", screen);
  // let mut image = screen.capture().unwrap();
  let image = screen.capture_area(0, 0, 200, 200).unwrap();

  let mut buffer = image.buffer();
  // print the first 10 bytes of the buffer
  println!("buffer: {:?}", &buffer[0..10]);
  fs::write(format!("screenshots/{}.png", screen.display_info.id), &buffer).unwrap();
  // print the number of bytes in the buffer
  println!("buffer size: {}", buffer.len());

  println!("Time taken: {:?}", start.elapsed());
}

fn main() {
    take_screenshot();
    println!("Starting mouse wiggler");
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'A' as u32,
        || take_screenshot(),
    )
    .unwrap();

    hk.listen();
    // wait for 5 seconds
    sleep(Duration::from_secs(5));
}
