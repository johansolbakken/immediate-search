mod text_import;
mod ui;

use std::collections::HashSet;

use raylib::prelude::*;

fn find_files_recursive(directory: &str) -> Vec<String> {
    let mut files = vec![];
    let paths = std::fs::read_dir(directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let mut sub_files = find_files_recursive(path.to_str().unwrap());
            files.append(&mut sub_files);
        } else {
            let path = path.to_str().unwrap().to_string();
            files.push(path);
        }
    }

    files
}

fn main() {
    let directory = "/Users/johansolbakken/Downloads/papers";
    let mut files = HashSet::<String>::new();

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    rl.set_exit_key(None);

    let mut ui = ui::UI::new();

    let mut search_root = "".to_string();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let x = 10;
        let mut y = 10;
        let margin = 10;

        d.clear_background(Color::new(9, 38, 53, 255));

        if ui.text_input(&mut d, 6, &mut search_root, x, y, 200, 50) {
            /*for file in files.into_iter() {
                let start = std::time::Instant::now();

                let path = format!("{}/{}", directory, file);
                let text = text_import::read_pdf(&path);

                let elapsed = start.elapsed();
                println!("{}: {}ms", file, elapsed.as_millis());
            }*/

            let new_files = find_files_recursive(&search_root);
            for file in new_files {
                files.insert(file);
            }
        }
        y += 50 + margin;

        for file in files.iter() {
            if ui.button(&mut d, 1, file, x, y, 200, 50) {
                
            }
            y += 50 + margin;
        }
    }
}
