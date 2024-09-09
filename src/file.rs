use chrono::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use crate::err::{Result, Error};
use actix_multipart::Field;
use futures::TryStreamExt;
use tokio::{
    fs,
    io::AsyncWriteExt
};
use std::path::{Path, PathBuf};


#[derive(Debug, Clone)]
pub struct FileSaver<'a> {
    dir: &'a str,
    allowed_fmts: Option<Vec<&'a str>>,
    max_size: usize
}

impl<'a> FileSaver<'a> {
    pub fn new<T>(
        dir: &'a str,
        allowed_fmts: impl Into<Option<T>>,
        max_size: usize
        ) -> Self
        where
            T: IntoIterator<Item = &'a str>
    {
        Self::default()
        .dir(dir)
        .allowed_fmts(allowed_fmts)
        .max_size(max_size)
    }

    pub async fn save_bytes_checked(&self, bytes: &[u8], filename: &str) -> Result<File> {
        if bytes.len() > self.max_size {
            return Err(Error::FileTooLarge(self.max_size));
        }

        let ext = self.get_ext_checked(filename)?;
        let (new_filename, path) = self.create_file_path(ext);

        let mut file = fs::File::create(&path).await?;
        file.write_all(bytes).await?;

        Ok(File::new(new_filename, self.dir.to_string()))
    }

    pub async fn save_bytes(&self, bytes: &[u8], filename: &str) -> Result<File> {
        let ext = match Self::get_ext(filename) {
            Some(x) => x,
            None => return Err(Error::InvalidFile)
        };

        let (new_filename, path) = self.create_file_path(ext);

        let mut file = fs::File::create(&path).await?;
        file.write_all(bytes).await?;

        Ok(File::new(new_filename, self.dir.to_string()))
    }

    pub async fn copy(&self, src: impl AsRef<Path>, filename: &str) -> Result<File> {
        let ext = match Self::get_ext(filename) {
            Some(x) => x,
            None => return Err(Error::InvalidFile)
        };

        let (new_filename, path) = self.create_file_path(ext);

        fs::copy(src, path).await?;

        Ok(File::new(new_filename, self.dir.to_string()))
    }

    pub async fn save_field(self, field: &mut Field) -> Result<File> {
        let filename: &str = match field.content_disposition().get_filename() {
            Some(f) => f,
            None => return Err(Error::InvalidFile)
        };

        let ext = self.get_ext_checked(filename)?;
        let (new_filename, path) = self.create_file_path(ext);

        let mut size: usize = 0;
        let mut file = fs::File::create(&path).await?;
        while let Ok(Some(chunk)) = field.try_next().await {
            size += chunk.len();
            if size > self.max_size {
                fs::remove_file(&path).await?;
                return Err(Error::FileTooLarge(self.max_size));
            }
            file.write_all(&chunk).await?;
        }

        Ok(File::new(new_filename, self.dir.to_string()))
    }

    fn create_file_path(&self, ext: &str) -> (String, PathBuf) {
        let filename = Self::generate_filename(ext);
        let mut path = PathBuf::from(self.dir);
        path.push(&filename);

        (filename, path)
    }

    fn get_ext_checked<'b>(&self, filename: &'b str) -> Result<&'b str> {
        let ext = match Self::get_ext(filename) {
            Some(x) => x,
            None => return Err(Error::InvalidFile)
        };

        if let Some(ref allowed_fmts) = &self.allowed_fmts {
            if !allowed_fmts.contains(&ext) {
                return Err(Error::InvalidFile);
            }
        }

        Ok(ext)
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
 

    pub fn get_ext(filename: &str) -> Option<&str> {
        for (i, c) in filename.chars().into_iter().rev().enumerate() {
            if c == '.' {
                return Some(&filename[filename.len()-i..])
            }
        }

        None
    }

    pub fn dir(mut self, dir: &'a str) -> Self {
        self.dir = dir;
        self
    }

    pub fn allowed_fmts<T>(
        mut self,
        allowed_fmts: impl Into<Option<T>>
        ) -> Self
        where
            T: IntoIterator<Item = &'a str>
    {
        let allowed_fmts = allowed_fmts.into();

        self.allowed_fmts = match allowed_fmts {
            None => None,
            Some(new_fmts) => {
                let mut v = Vec::new();

                for fmt in new_fmts.into_iter() {
                    v.push(fmt);
                }

                Some(v)
            }
        };

        self
    }

    pub fn max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }
}

const MAX_SIZE: usize = 50 * 1024 * 1024;

impl<'a> Default for FileSaver<'a> {
    fn default() -> Self {
        Self {
            dir: "./",
            allowed_fmts: None,
            max_size: MAX_SIZE
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    filename: String,
    dir: String
}

impl File {
    pub fn new(filename: String, dir: String) -> Self {
        Self { filename, dir }
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn filename_owned(self) -> String {
        self.filename
    }

    pub fn dir(&self) -> &String {
        &self.dir
    }

    pub async fn remove(&self) -> Result<()> {
        let mut path: PathBuf = PathBuf::from(&self.dir);
        path.push(&self.filename);
        fs::remove_file(&path).await?;

        Ok(())
    }

    pub fn saver<'a>() -> FileSaver<'a> {
        FileSaver::default()
    }
}

