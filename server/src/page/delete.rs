use actix_web::{Error, HttpRequest, HttpResponse};
use shared::{JsonStruct, Folder, FType};

pub async fn deletef(req: HttpRequest) -> Result<HttpResponse, Error> {
    let to_delete = crate::lib::http::without_cli(req.path());
    let mut result = JsonStruct {
        result: false,
        lenght: 0,
        ftype: FType::File,
        content: vec![],
    };
    match std::fs::remove_dir(to_delete) {
        Ok(_o) => {
            result.result = true;
            result.content = vec![Folder{
                result: true,
                name: "Work".to_string(),
                ftype: FType::File
            }]
        }
        Err(_e) => result.content = vec![Folder{
            result: false,
            name: "Error".to_string(),
            ftype: FType::Error
        }],
    };
    Ok(HttpResponse::Ok()
        .header("charset", "utf-8")
        .header("Access-Control-Allow-Origin", "*")
        .body(serde_json::to_string(&result).unwrap()))
}