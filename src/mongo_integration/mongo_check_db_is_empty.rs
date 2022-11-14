use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::common::where_was::WhereWas;
use tufa_common::config_mods::source_place_type::SourcePlaceType;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    ImplGetSourceWithMethodFromTufaCommon,
    ImplDisplayForErrorStruct,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
)]
pub struct MongoCheckDbIsEmptyError {
    source: MongoCheckDbIsEmptyErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethodFromTufaCommon, ImplDisplayForSimpleErrorEnum)]
pub enum MongoCheckDbIsEmptyErrorEnum {
    ClientWithOptions(mongodb::error::Error),
    ListCollectionNames(mongodb::error::Error),
    NotEmpty(usize),
    DatabaseDrop(mongodb::error::Error),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_db_is_empty(
    client_options: ClientOptions,
    db_name: &str,
    source_place_type: &SourcePlaceType,
    should_trace: bool,
) -> Result<(), Box<MongoCheckDbIsEmptyError>> {
    match Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            MongoCheckDbIsEmptyError::init_error_with_possible_trace(
                MongoCheckDbIsEmptyErrorEnum::ClientWithOptions(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        )),
        Ok(client) => match client.database(db_name).list_collection_names(None).await {
            Err(e) => Err(Box::new(
                MongoCheckDbIsEmptyError::init_error_with_possible_trace(
                    MongoCheckDbIsEmptyErrorEnum::ListCollectionNames(e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    source_place_type,
                    &GIT_INFO,
                    should_trace,
                ),
            )),
            Ok(documents_number) => {
                if !documents_number.is_empty() {
                    return Err(Box::new(
                        MongoCheckDbIsEmptyError::init_error_with_possible_trace(
                            MongoCheckDbIsEmptyErrorEnum::NotEmpty(documents_number.len()),
                            WhereWas {
                                time: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("cannot convert time to unix_epoch"),
                                location: *core::panic::Location::caller(),
                            },
                            source_place_type,
                            &GIT_INFO,
                            should_trace,
                        ),
                    ));
                }
                Ok(())
            }
        },
    }
}
