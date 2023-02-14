pub fn dev_with_deserialize() {
    let _f = one_with_deserialize();
    if let Err(e) = _f {
        println!("{}", e);
    }
}
//from implementation was not generated by thiserror with 'a lifetime https://github.com/dtolnay/thiserror/issues/68

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum OneWrapperErrorWithDeserialize<'a> {
    Something {
        #[serde(borrow)]
        inner_error: OneWrapperErrorEnumWithDeserialize<'a>,
        #[serde(borrow)]
        code_occurence:
            tufa_common::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a>,
    },
}

impl<'a> std::fmt::Display for OneWrapperErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a> tufa_common::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfigLifetime<'a> for OneWrapperErrorWithDeserialize<'a> {
    fn source_to_string_without_config_lifetime(&self) -> String {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            OneWrapperErrorWithDeserialize::Something { inner_error, code_occurence: _code_occurence } => inner_error.to_string_without_config_lifetime_with_deserialize(),
        }
    }
}

impl<'a> tufa_common::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceLifetimeWithDeserialize<'a>
    for OneWrapperErrorWithDeserialize<'a>
{
    fn get_code_occurence_lifetime_with_deserialize(
        &self,
    ) -> &tufa_common::common::code_occurence::CodeOccurenceLifetimeWithDeserialize<'a> {
        match self {
            OneWrapperErrorWithDeserialize::Something {
                inner_error: _inner_error,
                code_occurence,
            } => code_occurence,
        }
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum OneWrapperErrorEnumWithDeserialize<'a> {
    #[serde(borrow)]
    ThreeWrapper(tufa_common::dev_with_deserialize::ThreeWrapperErrorWithDeserialize<'a>),
}

impl<'a> std::fmt::Display for OneWrapperErrorEnumWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        write!(
            f,
            "{}",
            self.to_string_without_config_lifetime_with_deserialize()
        )
    }
}

impl<'a>
    tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize<
        'a,
    > for OneWrapperErrorEnumWithDeserialize<'a>
{
    fn to_string_without_config_lifetime_with_deserialize(&self) -> String {
        use tufa_common::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigLifetimeWithDeserialize;
        match self {
            OneWrapperErrorEnumWithDeserialize::ThreeWrapper(i) => {
                i.to_string_without_config_lifetime_with_deserialize()
            }
        }
    }
}

pub fn one_with_deserialize<'a>() -> Result<(), Box<OneWrapperErrorWithDeserialize<'a>>> {
    if let Err(e) = tufa_common::dev_with_deserialize::three_with_deserialize() {
        return Err(Box::new(OneWrapperErrorWithDeserialize::Something {
            inner_error:
                crate::dev_with_deserialize::OneWrapperErrorEnumWithDeserialize::ThreeWrapper(*e),
            code_occurence:
                tufa_common::common::code_occurence::CodeOccurenceLifetimeWithDeserialize::new(
                    &tufa_common::global_variables::compile_time::git_info::GIT_INFO,
                    file!(),
                    line!(),
                    column!(),
                ),
        }));
    }
    Ok(())
}
