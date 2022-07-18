use actix_web::{web, App, HttpResponse, HttpServer};


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {

    let path = Path::new(filepath);
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    

    for line in reader.lines() {
        println!("{}", line?);
    }














    

    Ok(())
}






fn main() {



    let path = "src/random_words/4-letters.txt";
  let result =  read_file_line_by_line(path.trim());
  println!("{:?}", result);
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get())
            .route("/word", web::post())
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server")
}

