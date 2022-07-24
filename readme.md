# Random Article Title and Text Generator

This is a simple project to learn more about Rust. It's an Actix web server with an API for generating random article titles and subtitles and text, and also random words, for when one needs filler text when developing a website. Titles and subtitles are 12 words; full text articles are 1,200 words. 

## Routes
 - /word/{number}: Generates a random word in JSON format {"word": "random_word"}. {number} determines how many letters the word will have (3 to 15);

 - /article/{number}: Generates a JSON article object with properties with randomly generated values, "title", "subtitle", and "date". 

 - /fulltext/{number}: Generates a JSON article object with properties with randomly generated values, "title", "subtitle", "fulltext", and "date". 

## Try it out 
The API is deployed to Heroku using this buildpack, https://github.com/emk/heroku-buildpack-rust, here:

 - https://gentle-sea-27356.herokuapp.com/
