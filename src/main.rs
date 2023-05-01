use actix_web::{web, App, HttpResponse, HttpServer};
use rand::Rng;
use serde_json::json;

///Takes a String from the url and uses it as input.
///Creates a JSON response with several values derived from the input.
async fn count_letters(word: web::Path<String>) -> HttpResponse {
    let word = word.into_inner();
    let ltr_count = word.len();
    let index = rand::thread_rng().gen_range(0..ltr_count);
    let rnd_letter = word.chars().nth(index).unwrap();
    let rnd_letter_ascii = rnd_letter as u32;
    let reversed: String = word.chars().rev().collect();
    let ascii_codes: Vec<u32> = word.chars().map(|c| c as u32).collect();
    let ascii_sum: u32 = ascii_codes.iter().sum();

    HttpResponse::Ok().json(json!({
        "letterCount": ltr_count,
        "randomLetter": rnd_letter,
        "randomLetterAscii": rnd_letter_ascii,
        "reversedWord": reversed,
        "lettersAsciiCodes":ascii_codes,
        "sumOfLetterAsciiCodes": ascii_sum
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/word/{word}", web::get().to(count_letters)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
