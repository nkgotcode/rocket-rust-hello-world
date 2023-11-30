#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;
use rocket::Data;
use rocket::http::ContentType;
use multipart::server::Multipart;
use std::io::Read;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/upload-audio", data = "<data>")]
async fn upload_audio(content_type: &ContentType, data: Data<'_>) -> Result<String, rocket::response::status::Custom<String>> {
    let boundary = content_type.params().find(|&(k, _)| k == "boundary").map(|(_, v)| v.to_string());
    if let Some(boundary) = boundary {
        let mut multipart = Multipart::with_body(data.open(), boundary);
        // Process each field in the form
        while let Ok(Some(mut field)) = multipart.read_entry() {
            if field.headers.name == "audio" {
                // Handle audio file
                let mut audio_buffer = Vec::new();
                field.data.read_to_end(&mut audio_buffer)?;
                // Process audio_buffer as needed
            }
            // Handle other form fields if necessary
        }
        Ok("Audio uploaded successfully".to_string())
    } else {
        Err(rocket::response::status::Custom(
            rocket::http::Status::BadRequest,
            "Missing content-type boundary".to_string(),
        ))
    }
}

fn main() {
    let rocket = rocket::ignite();
    rocket.mount("/", routes![hello]);
    rocket.mount("/upload-audio", routes![upload_audio]);
    rocket.launch();
}
