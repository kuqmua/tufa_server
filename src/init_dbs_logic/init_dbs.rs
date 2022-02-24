use super::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use super::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub enum InitDbsErrorEnum {
    InitDbsProvidersLinkParts {
        source: InitDbsProvidersLinkPartsError,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_dbs() -> Result<(), Box<InitDbsErrorEnum>> {
    if let Err(e) = init_dbs_with_providers_link_parts().await {
        return Err(Box::new(InitDbsErrorEnum::InitDbsProvidersLinkParts {
            source: e,
            where_was: WhereWas {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
