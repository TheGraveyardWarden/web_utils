use chrono::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use crate::err::{Result, Error};
use actix_multipart::Field;
use futures::TryStreamExt;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;


pub fn get_ext(filename: &str) -> Option<&str> {
    for (i, c) in filename.chars().into_iter().rev().enumerate() {
        if c == '.' {
            return Some(&filename[filename.len()-i..])
        }
    }

    None
}

pub fn generate_filename(ext: &str) -> String {
    let time: String = Local::now().format("%Y-%m-%d--%H-%M-%S").to_string();
    let r: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    format!("{r}-{time}.{ext}")
}

pub async fn save_file(field: &mut Field, allowed_fmts: &[&str], dir: &str) -> Result<String> {
    let filename: &str = match field.content_disposition().get_filename() {
        Some(n) => n,
        None => return Err(Error::InvalidFile)
    };

    let file_ext: &str = match get_ext(filename) {
        Some(ext) => ext,
        None => return Err(Error::InvalidFile)
    };

    if !allowed_fmts.contains(&file_ext) {
        return Err(Error::InvalidFile);
    }

    let file_path: String = generate_filename(file_ext);
    let mut path: PathBuf = PathBuf::from(&dir);
    path.push(&file_path);

    let mut file = fs::File::create(&path).await?;
    while let Ok(Some(chunk)) = field.try_next().await {
        file.write_all(&chunk).await?;
    }

    Ok(file_path)
}

pub async fn remove_file(file_path: &str, dir: &str) -> Result<()> {
    let mut path: PathBuf = PathBuf::from(&dir);
    path.push(&file_path);
    fs::remove_file(&path).await?;
    Ok(())
}
