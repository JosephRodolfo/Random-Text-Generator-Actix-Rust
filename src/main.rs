// use actix_web::http::header::Date;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use chrono::{DateTime, NaiveDateTime, Utc};
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use uuid::Uuid;
#[derive(Debug, Serialize)]
struct ArticleTitle {
    title: String,
    subtitle: String,
    date: String,
    text: String,
    id: String,
}

// impl ArticleTitle {
//     fn new() -> Self {
//         Object(OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst))
//     }
// }

#[derive(Debug, Serialize)]
struct RandomWord {
    word: String,
}

fn create_random_date() -> DateTime<Utc> {
    let rand_time = generate_random_number(1, 31_556_952);
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(1_000_000_000 + rand_time as i64, 0),
        Utc,
    );
    dt
}

fn generate_random_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(min..max);
    return number;
}

fn get_random_word(number: u8) -> RandomWord {
    let path = format!("src/random_words/{}-letters.txt", number.to_string());
    let rand_word = read_file_line_by_line(&path);
    let single_word = &rand_word[0];

    let word_obj = RandomWord {
        word: single_word.to_string(),
    };
    word_obj
}

fn get_word(req: HttpRequest) -> HttpResponse {
    // HttpResponse::Ok().content_type("text/html").body("Test")
    let num_letters: u8 = req.match_info().get("number").unwrap().parse().unwrap();
    let word = get_random_word(num_letters);
    HttpResponse::Ok().json(word)
}

fn get_article(req: HttpRequest) -> HttpResponse {
    let mut articles: Vec<ArticleTitle> = Vec::new();
    let num_articles: u8 = req.match_info().get("number").unwrap().parse().unwrap();

    for _i in 0..num_articles {
        let article_obj = get_title_subtitle();
        articles.push(article_obj);
    }
    HttpResponse::Ok().json(articles)
}

fn get_fulltext(req: HttpRequest) -> HttpResponse {
    let mut articles: Vec<ArticleTitle> = Vec::new();
    let num_articles: u8 = req.match_info().get("number").unwrap().parse().unwrap();

    for _i in 0..num_articles {
        let mut article_obj = get_title_subtitle();
        let text = create_paragraph();
        article_obj.text = text;
        articles.push(article_obj);
    }
    HttpResponse::Ok().json(articles)
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn create_title(title: String) -> String {
    let mut vec: Vec<String> = Vec::new();
    let random = generate_random_number(6, 12);
    for _i in 0..random {
        let word_length = generate_random_number(3, 15);

        let path = format!("src/random_words/{}-letters.txt", word_length.to_string());

        let mut result = read_file_line_by_line(&path);
        vec.append(&mut result);
    }
    vec.shuffle(&mut thread_rng());
    let mut new_string = vec;
    let final_tital = match title.as_str() {
        "title" => {
            let result = new_string
                .iter()
                .map(|e| some_kind_of_uppercase_first_letter(e))
                .collect::<Vec<String>>()
                .join(" ");
            return result;
        }
        "subtitle" => {
            new_string[0] = some_kind_of_uppercase_first_letter(&new_string[0]);
            let result = new_string.join(" ");
            return result;
        }
        _ => "",
    };

    final_tital.to_string()
}

fn get_title_subtitle() -> ArticleTitle {
    let title = create_title("title".to_string());
    let subtitle = create_title("subtitle".to_string());
    let date = create_random_date().to_string();
    let id = create_uuid().to_string();
    let article = ArticleTitle {
        title,
        subtitle,
        date,
        text: "".to_string(),
        id,
    };
    article
}

fn create_uuid() -> Uuid {
    let id = Uuid::new_v4();
    id
}

fn read_file_line_by_line(filepath: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(&file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));

    let random = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");

    vec.push(random);

    vec
}

fn create_paragraph() -> String {
    let mut random = Vec::new();

    for i in 3..15 {
        let path = format!("src/random_words/{}-letters.txt", i.to_string());

        let path = Path::new(&path);
        let file = File::open(path).expect("Cannot open file.txt");
        let reader = BufReader::new(&file);
        let lines = reader.lines().map(|l| l.expect("Couldn't read line"));
        let mut new_words = lines.choose_multiple(&mut rand::thread_rng(), 100);
        random.append(&mut new_words);
    }

    random.shuffle(&mut thread_rng());
    let mut new_arr = random;
    let length = new_arr.len();
    new_arr[0] = some_kind_of_uppercase_first_letter(&new_arr[0]);
    new_arr[length - 1] = format!("{}.", new_arr[length - 1]);

    for i in 10..length {
        if i % 15 == 0 {
            new_arr[i - 1] = format!("{}.", new_arr[i - 1]);
            new_arr[i] = some_kind_of_uppercase_first_letter(&new_arr[i]);
        }
    }

    let para_string = new_arr.join(" ");
    para_string
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/word/{number}", web::get().to(get_word))
            .route("/article/{number}", web::get().to(get_article))
            .route("/fulltext/{number}", web::get().to(get_fulltext))
    });
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server")
}
