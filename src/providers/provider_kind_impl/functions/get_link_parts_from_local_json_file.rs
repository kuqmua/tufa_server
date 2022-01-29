use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub struct GetLinkPartsFromLocalJsonFileError {
    pub source: Box<GetLinkPartsFromLocalJsonFileErrorEnum>,
    line: String,
}

#[derive(Debug)]
pub enum GetLinkPartsFromLocalJsonFileErrorEnum {
    TokioFsFileOpen(TokioFsFileOpenErrorStruct),
    TokioIoAsyncReadExtReadToEnd(TokioIoAsyncReadExtReadToEndErrorStruct),
    SerdeJsonFromSlice(serde_json::Error),
}

#[derive(Debug)]
pub struct TokioFsFileOpenErrorStruct {
    source: std::io::Error,
    line: String,
}

#[derive(Debug)]
pub struct TokioIoAsyncReadExtReadToEndErrorStruct {
    source: std::io::Error,
    line: String,
}

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn get_link_parts_from_local_json_file(
        self,
    ) -> Result<Vec<String>, GetLinkPartsFromLocalJsonFileError> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => Err(GetLinkPartsFromLocalJsonFileError {
                source: Box::new(GetLinkPartsFromLocalJsonFileErrorEnum::TokioFsFileOpen(
                    TokioFsFileOpenErrorStruct {
                        source: e,
                        line: format!("{}:{}:{}", line!(), file!(), column!()),
                    },
                )),
                line: format!("{}:{}:{}", line!(), file!(), column!()),
            }),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(GetLinkPartsFromLocalJsonFileError {
                        source: Box::new(
                            GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd(
                                TokioIoAsyncReadExtReadToEndErrorStruct {
                                    source: e,
                                    line: format!("{}:{}:{}", line!(), file!(), column!()),
                                },
                            ),
                        ),
                        line: format!("{}:{}:{}", line!(), file!(), column!()),
                    });
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(GetLinkPartsFromLocalJsonFileError {
                        source: Box::new(
                            GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice(e),
                        ),
                        line: format!("{}:{}:{}", line!(), file!(), column!()),
                    }),
                    Ok(file_content_as_struct) => Ok(file_content_as_struct.data),
                }
            }
        }
    }
}
