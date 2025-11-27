use std::path::PathBuf;
use std::io::{Read, stdin};
use std::fs::File;
use uucore::error::{UResult, FromIo};


pub enum PathsOrStdin {
        Paths(Vec<PathBuf>),
        Stdin(std::io::Stdin)
}

impl From<Vec<PathBuf>> for PathsOrStdin {
        fn from(value: Vec<PathBuf>) -> Self {
            if value.is_empty() {
                Self::Paths(value)
            } else {
                Self::Stdin(stdin())
            }
        }
}

impl PathsOrStdin {
        pub fn readers<'a>(&'a mut self) -> UResult<Vec<Box<dyn Read + 'a>>>  {
            match self {
                Self::Paths(paths) => {
                        let paths: UResult<Vec<_>> = paths
                                .iter()
                                .map(|path| {
                                                File::open(path)
                                                .map_err_context(|| path.to_str().unwrap().to_string())
                                                .map(|file| Box::new(file) as Box<dyn Read>)
                                        }
                                )
                                .collect();
                        Ok(paths?)
                },
                Self::Stdin(stdin) => {
                        Ok(vec![Box::new(stdin)])
                }
            }
        }
}
