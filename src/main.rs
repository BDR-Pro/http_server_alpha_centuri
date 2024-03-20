#[macro_use]
extern crate rocket;
use rocket::data::{Data, ToByteUnit};
use rocket::http::ContentType;
use std::fs::File;
use std::io::Write;
use rocket::{fairing::{AdHoc}, Build, Rocket};
use std::path::PathBuf;
use uuid::Uuid;
use rocket_dyn_templates::Template;
use std::collections::HashMap;



#[get("/<payment_id>")]
fn upload_instruction(payment_id: &str) -> Template {
    let mut context = HashMap::new();
    context.insert("payment_id", payment_id.to_string());
    Template::render("payment", &context) 
}


// Route to handle the file upload
#[post("/<payment_id>", data = "<file_data>")]
async fn upload_file(
    payment_id: &str,
    _content_type: &ContentType,
    file_data: Data<'_>,
) -> std::io::Result<String> {
    let dir = "./uploaded_files";
    std::fs::create_dir_all(dir)?;

    let file_name = format!("{}{}.pem", payment_id, Uuid::new_v4());
    let file_path = PathBuf::from(dir).join(file_name);

    let mut file = File::create(&file_path)?;
    let limit = file_data.open(10.mebibytes());
    let stream = limit.into_bytes().await;
    match stream {
        Ok(data) => {
            if data.is_complete() {
                file.write_all(&data.into_inner())?;
                Ok(format!("File uploaded successfully: {:?}", file_path))
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Data is too large",
                ))
            }
        }
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )),
    }
}
#[get("/")]
fn hello_world() -> &'static str {
    "<h1> Hello, world! welcome to the payment server of the future!
     Thank you for using alpha centuri payment system!
     use the /<payment_id> route to upload your payment 
     file using POST method! </h1>"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .configure(rocket::Config {
            port: 8080,
            ..rocket::Config::default()
        })
        // Mount routes
        .attach(Template::fairing())
        .mount("/", routes![upload_instruction, upload_file, hello_world])
    
}

