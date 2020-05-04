#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod paste_id;

use paste_id::PasteId;

use std::io;
use std::path::Path;

use rocket::Data;
use rocket::http::RawStr;
use std::fs::File;

fn main()
{
    rocket::ignite()
        .mount("/", routes![index, upload, retrieve])
        .mount("/remove", routes![delete])
        .launch();
}

#[get("/")]
fn index() -> &'static str
{
    "
            USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error>
{
    let id = PasteId::new(3);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File>
{
    let filename =  format!("upload/{id}", id=id);
    File::open(&filename).ok()
}

#[delete("/<id>")]
fn delete(id: PasteId) -> String
{
    let filename = format!("upload/{id}", id = id);
    match std::fs::remove_file(&filename)
    {
        Ok(f) => format!("{} Removed\n", &filename),
        Err(e) => format!("Something went wrong {}\n", e)
    }

}