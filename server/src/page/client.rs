use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::Error;
const CLIENT_PAGE: &str = "./client/index.html";

pub async fn client() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open(CLIENT_PAGE)?.set_status_code(StatusCode::NOT_FOUND))
}
