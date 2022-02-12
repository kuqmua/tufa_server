use std::fmt;
use std::path::Path;

use tokio::io::AsyncWriteExt;

use crate::helpers::where_was::WhereWas;
//
#[derive(thiserror::Error, Debug, ImplDisplayDerive)]
pub struct WriteStringIntoFileWithTokioError {
    pub source: Box<WriteStringIntoFileWithTokioErrorEnum>,
}

#[derive(thiserror::Error, Debug, ImplDisplayDerive)]
pub enum WriteStringIntoFileWithTokioErrorEnum {
    StdFsCreateDirAll {
        source: std::io::Error,
        where_was: WhereWas,
    },
    TokioFsFileOpen {
        source: std::io::Error,
        where_was: WhereWas,
    },
    FileWriteAll {
        source: std::io::Error,
        where_was: WhereWas,
    },
}

pub async fn write_string_into_file_with_tokio(
    path: &Path,
    stringified_json: String,
) -> Result<(), WriteStringIntoFileWithTokioError> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(WriteStringIntoFileWithTokioError {
                source: Box::new(WriteStringIntoFileWithTokioErrorEnum::StdFsCreateDirAll {
                    source: e,
                    where_was: WhereWas {
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                }),
            });
        }
    }
    match tokio::fs::File::open(path).await {
        Err(e) => Err(WriteStringIntoFileWithTokioError {
            source: Box::new(WriteStringIntoFileWithTokioErrorEnum::TokioFsFileOpen {
                source: e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }),
        }),
        Ok(mut file) => {
            if let Err(e) = file.write_all(stringified_json.as_bytes()).await {
                return Err(WriteStringIntoFileWithTokioError {
                    source: Box::new(WriteStringIntoFileWithTokioErrorEnum::FileWriteAll {
                        source: e,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                });
            }
            Ok(())
        }
    }
}
