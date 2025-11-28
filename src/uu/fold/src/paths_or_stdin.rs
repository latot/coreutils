use std::path::PathBuf;
use std::io::{Read, stdin, Stdin};
use std::fs::File;
use std::str::FromStr;
use uucore::error::{UResult, FromIo, UError};

#[derive(Debug)]
pub enum PathOrStdin {
        Path(PathBuf),
        Stdin(Stdin),
}

impl PathOrStdin {
        pub fn as_readers<'a>(&'a self) -> UResult<Box<dyn Read + 'a>>  {
                match self {
                        Self::Path(pathbuf) => {
                                File::open(pathbuf)
                                .map_err_context(|| pathbuf.to_str().unwrap().to_string())
                                .map(|file| Box::new(file) as Box<dyn Read>)
                        },
                        Self::Stdin(stdin) => {
                                Ok(Box::new(stdin))
                        }
                }
        }
}

impl FromStr for PathOrStdin {
        type Err = Box<dyn std::error::Error>;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s == "-" {
                Ok(Self::Stdin(stdin()))
            } else {
                Ok(Self::Path(PathBuf::from_str(s)?))
            }
        }
}

pub struct PathsOrStdin(Vec<PathOrStdin>);

impl TryFrom<Vec<String>> for PathsOrStdin {
        type Error = Box<dyn std::error::Error>;
        fn try_from(mut value: Vec<String>) -> Result<Self, Self::Error> {
            value.remove(0);
            if value.is_empty() {
                Ok(Self(vec![PathOrStdin::Stdin(stdin())]))
            } else {
                let value = value
                .into_iter()
                .map(|x| PathOrStdin::from_str(&x))
                .collect::<Result<Vec<_>, Self::Error>>()?;
                Ok(Self(value))
            }
        }
}

impl PathsOrStdin {
        pub fn as_readers<'a>(&'a mut self) -> UResult<Vec<Box<dyn Read + 'a>>>  {
            let readers = self.0.iter().map(|reader| reader.as_readers()).collect::<UResult<Vec<_>>>()?;
            Ok(readers)
        }
}
