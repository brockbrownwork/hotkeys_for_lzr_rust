use rsautogui::mouse;
// use rsautogui::keyboard;
use rsautogui::screen;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use hotkey;
use screenshots::Screen;
use std::{fs, time::Instant};

fn wiggle_mouse() {
    // print the current mouse position
    let (mouse_x, mouse_y) = mouse::position();
    println!("Mouse is at ({}, {})", mouse_x, mouse_y);
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
    // save a screenshot
    // make a tuple of (0, 0, 500, 500) in u16 format
    let x:u16 = 0;
    let y:u16 = 0;
    let w:u16 = 500;
    let h:u16 = 500;
    let screenshot = screen::screenshot(x, y, w, h);
    screen::printscreen(&screenshot, "screenshot.png");
}



fn take_screenshot() {
  let start = Instant::now();
  let screens = Screen::all().unwrap();

  for screen in screens {
    println!("capturer {:?}", screen);
    let mut image = screen.capture().unwrap();
    let mut buffer = image.buffer();
    fs::write(format!("target/{}.png", screen.display_info.id), &buffer).unwrap();

    image = screen.capture_area(300, 300, 300, 300).unwrap();
    buffer = image.buffer();
    fs::write(format!("target/{}-2.png", screen.display_info.id), &buffer).unwrap();
  }

  let screen = Screen::from_point(100, 100).unwrap();
  println!("capturer {:?}", screen);

  let image = screen.capture_area(300, 300, 300, 300).unwrap();
  let buffer = image.buffer();
  fs::write("target/capture_display_with_point.png", &buffer).unwrap();

  println!("Time taken: {:?}", start.elapsed());
}

fn main() {
    take_screenshot();
    println!("Starting mouse wiggler");
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'A' as u32,
        || wiggle_mouse(),
    )
    .unwrap();

    hk.listen();
    // wait for 5 seconds
    sleep(Duration::from_secs(5));
}
