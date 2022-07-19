use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use rand::prelude::IteratorRandom;
// use rand::Rng;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// fn generate_random_number(min: u16, max: u16) -> u16 {
//     let mut rng = rand::thread_rng();
//     let number: u16 = rng.gen_range(0..max);
//     return number;
// }

// fn get_line_count(length: u8) -> Option<u16> {
//     let total_lines = match length {
//         0 => {
//             let random = generate_random_number(3, 15);
//             return Some(random);
//         }

//         3 => return Some(1013),
//         4 => return Some(4029),
//         5 => return Some(8937),
//         6 => return Some(15787),
//         7 => return Some(24028),
//         8 => return Some(29765),
//         9 => return Some(29149),
//         10 => return Some(22325),
//         11 => return Some(16164),
//         12 => return Some(11416),
//         13 => return Some(7750),
//         14 => return Some(5058),
//         15 => return Some(3155),

//         _ => None,
//     };

//     return total_lines;
// }

fn get_random_word(number: u8) -> String {
    let path = format!("src/random_words/{}-letters.txt", number.to_string());
    let rand_word = read_file_line_by_line(&path);
    return rand_word;
}

// #[get("/word/{number}")]
fn get_word(req: HttpRequest) -> HttpResponse {
    // HttpResponse::Ok().content_type("text/html").body("Test")
    let num_letters: u8 = req.match_info().get("number").unwrap().parse().unwrap();
    let word = get_random_word(num_letters);
    HttpResponse::Ok().json(format!("word: {}", word))
}

fn read_file_line_by_line(filepath: &str) -> String {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));
    let random = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");
    return random;
}

fn main() {
    let server = HttpServer::new(|| App::new().route("/word/{number}", web::get().to(get_word)));

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server")
}
