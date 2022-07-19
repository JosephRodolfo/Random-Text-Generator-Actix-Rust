// use actix_web::http::header::Date;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use censor::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
#[derive(Debug, Serialize)]
struct ArticleTitle {
    title: String,
    subtitle: String,
    date: String,
}

fn create_random_date() -> DateTime<Utc> {
    let rand_time = generate_random_number(1, 31_556_952);
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(1_000_000_000 + rand_time as i64, 0),
        Utc,
    );
    return dt;
}

fn generate_random_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(min..max);
    return number;
}

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
    let single_word = &rand_word[0];
    return single_word.to_string();
}

// #[get("/word/{number}")]
fn get_word(req: HttpRequest) -> HttpResponse {
    // HttpResponse::Ok().content_type("text/html").body("Test")
    let num_letters: u8 = req.match_info().get("number").unwrap().parse().unwrap();
    let word = get_random_word(num_letters);
    HttpResponse::Ok().json(format!("word: {}", word))
}

fn get_article() -> HttpResponse {
    let article_obj = get_title_subtitle();
    return HttpResponse::Ok().json(article_obj);
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn get_title_subtitle() -> ArticleTitle {
    fn create_title() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        for i in 3..15 {
            let path = format!("src/random_words/{}-letters.txt", i.to_string());

            let mut result = read_file_line_by_line(&path);
            vec.append(&mut result);
        }
        vec.shuffle(&mut thread_rng());
        return vec;
    }

    let title = create_title();
    let mut subtitle = create_title();

    let uppper_case_title = title
        .iter()
        .map(|e| some_kind_of_uppercase_first_letter(e))
        .collect::<Vec<String>>();
    subtitle[0] = some_kind_of_uppercase_first_letter(&subtitle[0]);
    let joined_chars_title = &uppper_case_title.join(" ");
    let joined_chars_subtitle = &subtitle.join(" ");
    let date = create_random_date().to_string();

    let article = ArticleTitle {
        title: joined_chars_title.to_string(),
        subtitle: joined_chars_subtitle.to_string(),
        date,
    };
    println!("{:#?}", article);
    return article;
}

fn read_file_line_by_line(filepath: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(&file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));

    let mut random = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");
    let censor = Standard + Zealous + Sex;

    while censor.check(&random) {
        let reader = BufReader::new(&file);
        let lines = reader.lines().map(|l| l.expect("Couldn't read line"));

        random = lines
            .choose(&mut rand::thread_rng())
            .expect("File had no lines");
    }
    vec.push(random);
    return vec;
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/word/{number}", web::get().to(get_word))
            .route("/article", web::get().to(get_article))
    });
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server")
}
