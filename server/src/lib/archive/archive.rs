use crate::lib::file::file::get_file_as_byte_vec;
use actix_files::file_extension_to_mime;
use actix_http::Response;
use actix_utils::mpsc;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::web;
use async_std::fs as afs;
use bytes::Bytes;
use futures::AsyncReadExt;
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip_extensions::zip_create_from_directory_with_options;

pub async fn get_zip(path: String) -> std::io::Result<Response> {
    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(actix_web::web::Bytes::from(
        get_file_as_byte_vec(path.clone(), &"zip").await,
    )));
    Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .header(
            "Content-Disposition",
            format!(
                "\"attachment\";filename=\"{}.zip\"",
                path.clone().split('/').last().expect("Error")
            ),
        )
        .content_type(file_extension_to_mime(path.clone().as_str()).essence_str())
        .encoding(ContentEncoding::Gzip)
        .streaming(rx_body))
}

pub async fn get_tar(path: String) -> std::io::Result<Response> {
    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(actix_web::web::Bytes::from(
        get_file_as_byte_vec(path.clone(), &"tar").await,
    )));
    Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .header(
            "Content-Disposition",
            format!(
                "\"attachment\";filename=\"{}.tar.gz\"",
                path.clone().split('/').last().expect("Error")
            ),
        )
        .content_type(file_extension_to_mime(path.clone().as_str()).essence_str())
        .encoding(ContentEncoding::Gzip)
        .streaming(rx_body))
}

async fn async_zip_archive(name: String, dir: String) -> afs::File {
    let file_name = format!("./temp/{}.zip", name);
    File::create(file_name.clone()).unwrap();
    println!("filename => {}", dir);
    web::block(|| {
        zip_create_from_directory_with_options(
            &PathBuf::from(file_name),
            &PathBuf::from(dir),
            FileOptions::default().compression_method(CompressionMethod::Bzip2),
        )
    })
    .await
    .expect("Error");

    afs::File::open(format!("./temp/{}.zip", name))
        .await
        .expect("Error")
}

async fn async_tar_archive(name: String, dir: String) -> afs::File {
    let file_name = format!("./temp/{}.tar.gz", name);
    println!("{} dir : {}", file_name, dir);
    File::create(&file_name).expect("Error");
    let file = afs::File::open(&file_name);
    tar::Builder::new(File::open(&file_name).expect("no file found"))
        .append_dir_all(file_name.as_str(), dir.clone().as_str())
        .expect("Error");
    file.await.expect("Error")
}

pub async fn random_archive(extention: String, dir: String) -> afs::File {
    let name: String = random_name();
    let dir: &str = dir.as_ref();
    if extention == String::from("tar.gz") {
        async_tar_archive(name, dir.to_string()).await
    } else {
        async_zip_archive(name, dir.to_string()).await
    }
}

fn random_name() -> String {
    use rand::Rng;
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCEDFGHIJKLMNOPQRSTUVWXYZ123456789";
    let mut rng = rand::thread_rng();
    (0..10)
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect()
}

pub struct _ArchiveFile {
    pub src_path: String,
    pub name: String,
}

impl _ArchiveFile {
    pub async fn _read_zip(&'static self) -> std::io::Result<Bytes> {
        let file_name = format!("./temp/{}.zip", &self.name);
        let mut vec = Vec::new();
        let _ = afs::File::create(file_name.clone()).await?;
        println!("filename => {}", &self.src_path);
        let _ = web::block(move || {
            zip_create_from_directory_with_options(
                &PathBuf::from(file_name),
                &PathBuf::from(&self.src_path),
                FileOptions::default().compression_method(CompressionMethod::Bzip2),
            )
        })
        .await;

        let _ = afs::File::open(format!("./temp/{}.zip", &self.name))
            .await?
            .read_to_end(&mut vec)
            .await?;

        Ok(Bytes::from(vec))
    }
    pub fn _write_tar(&self) {}
    pub fn _read_to_bytes(&self) {}
}
