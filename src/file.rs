use chrono::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use crate::err::{Result, Error};
use actix_multipart::Field;
use futures::TryStreamExt;
use tokio::{
    fs,
    io::{AsyncWriteExt, AsyncReadExt}
};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub struct FileSaver<'a> {
    dir: &'a str,
    allowed_fmts: Option<Vec<&'a str>>,
}

impl<'a> FileSaver<'a> {
    pub fn new<T>(
        dir: &'a str,
        allowed_fmts: impl Into<Option<T>>
        ) -> Self
        where
            T: IntoIterator<Item = &'a str>
    {
        Self::default()
        .dir(dir)
        .allowed_fmts(allowed_fmts)
    }

    pub async fn save_field(self, field: &mut Field) -> Result<File> {
        let filename: &str = match field.content_disposition().get_filename() {
            Some(f) => f,
            None => return Err(Error::InvalidFile)
        };

        let ext = self.get_ext_checked(filename)?;
        let (new_filename, path) = self.create_file_path(ext);

        let mut file = fs::File::create(&path).await?;
        while let Ok(Some(chunk)) = field.try_next().await {
            file.write_all(&chunk).await?;
        }

        Ok(File::new(new_filename, self.dir.to_string()))
    }

    pub async fn save_path(self, filename: &str) -> Result<File> {
        let ext = self.get_ext_checked(filename)?;

        let mut src = fs::File::open(filename).await?;
        let mut contents = vec![];
        src.read_to_end(&mut contents).await?;
 
        let (new_filename, path) = self.create_file_path(ext);

        let mut dst = fs::File::create(&path).await?;
        dst.write_all(&contents).await?;

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
}

impl<'a> Default for FileSaver<'a> {
    fn default() -> Self {
        Self {
            dir: "./",
            allowed_fmts: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    filename: String,
    #[serde(skip_serializing)]
    dir: Option<String>
}

impl File {
    pub fn new(filename: String, dir: impl Into<Option<String>>) -> Self {
        Self {
            filename,
            dir: dir.into()
        }
    }

    pub fn set_dir(&mut self, dir: String) {
        self.dir = Some(dir);
    }

    pub async fn remove(self) -> Result<()> {
        if self.dir.is_none() {
            return Err(Error::DirNotSpecified);
        }
        let dir = self.dir.unwrap();

        let mut path: PathBuf = PathBuf::from(&dir);
        path.push(&self.filename);
        fs::remove_file(&path).await?;

        Ok(())
    }

    pub fn saver<'a>() -> FileSaver<'a> {
        FileSaver::default()
    }
}

