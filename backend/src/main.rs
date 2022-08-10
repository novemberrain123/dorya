use std::path::Path; 
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
  let page_directory_path = 
  format!("{}/../frontend/build", env!("CARGO_MANIFEST_DIR"));
  NamedFile::open(Path::new(&page_directory_path).join("index.html"))
}

