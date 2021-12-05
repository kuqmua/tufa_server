use procedural_macros_lib::AllVariants;
use procedural_macros_lib::EnumVariantCount;

use strum_macros::EnumIter;

#[derive(
    AllVariants,
    EnumVariantCount,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
)]
pub enum EnvBoolVar {
    MongoEnableInitialization,
    MongoEnableInitializationForProviders,
    MongoEnableInitializationForArxiv,
    MongoEnableInitializationForBiorxiv,
    MongoEnableInitializationForGithub,
    MongoEnableInitializationForHabr,
    MongoEnableInitializationForMedrxiv,
    MongoEnableInitializationForReddit,
    MongoEnableInitializationForTwitter,

    MongoEnableWriteErrorLogs,
    MongoEnableWriteErrorLogsForProviders,
    MongoEnableWriteErrorLogsForArxiv,
    MongoEnableWriteErrorLogsForBiorxiv,
    MongoEnableWriteErrorLogsForGithub,
    MongoEnableWriteErrorLogsForHabr,
    MongoEnableWriteErrorLogsForMedrxiv,
    MongoEnableWriteErrorLogsForReddit,
    MongoEnableWriteErrorLogsForTwitter,

    MongoEnableCleaningWarningLogsDb,
    MongoEnableCleaningWarningLogsDbForProviders,
    MongoEnableCleaningWarningLogsDbForArxiv,
    MongoEnableCleaningWarningLogsDbForBiorxiv,
    MongoEnableCleaningWarningLogsDbForGithub,
    MongoEnableCleaningWarningLogsDbForHabr,
    MongoEnableCleaningWarningLogsDbForMedrxiv,
    MongoEnableCleaningWarningLogsDbForReddit,
    MongoEnableCleaningWarningLogsDbForTwitter,

    MongoEnableCleaningWarningLogsDbCollections,
    MongoEnableCleaningWarningLogsDbCollectionsForProviders,
    MongoEnableCleaningWarningLogsDbCollectionsForArxiv,
    MongoEnableCleaningWarningLogsDbCollectionsForBiorxiv,
    MongoEnableCleaningWarningLogsDbCollectionsForGithub,
    MongoEnableCleaningWarningLogsDbCollectionsForHabr,
    MongoEnableCleaningWarningLogsDbCollectionsForMedrxiv,
    MongoEnableCleaningWarningLogsDbCollectionsForReddit,
    MongoEnableCleaningWarningLogsDbCollectionsForTwitter,

    MongoEnableLinkPartsRandomizeOrder,
    MongoEnableLinkPartsRandomizeOrderForProviders,
    MongoEnableLinkPartsRandomizeOrderForArxiv,
    MongoEnableLinkPartsRandomizeOrderForBiorxiv,
    MongoEnableLinkPartsRandomizeOrderForGithub,
    MongoEnableLinkPartsRandomizeOrderForHabr,
    MongoEnableLinkPartsRandomizeOrderForMedrxiv,
    MongoEnableLinkPartsRandomizeOrderForReddit,
    MongoEnableLinkPartsRandomizeOrderForTwitter,

    EnablewriteErrorLogsInLocalFolder,
    EnablewriteErrorLogsInLocalFolderForProvider,
    EnablewriteErrorLogsInLocalFolderForArxiv,
    EnablewriteErrorLogsInLocalFolderForBiorxiv,
    EnablewriteErrorLogsInLocalFolderForGithub,
    EnablewriteErrorLogsInLocalFolderForHabr,
    EnablewriteErrorLogsInLocalFolderForMedrxiv,
    EnablewriteErrorLogsInLocalFolderForReddit,
    EnablewriteErrorLogsInLocalFolderForTwitter,

    EnableCleaningWarningLogsDirectory,
    EnableCleaningWarningLogsDirectoryForProviders,
    EnableCleaningWarningLogsDirectoryForArxiv,
    EnableCleaningWarningLogsDirectoryForBiorxiv,
    EnableCleaningWarningLogsDirectoryForGithub,
    EnableCleaningWarningLogsDirectoryForHabr,
    EnableCleaningWarningLogsDirectoryForMedrxiv,
    EnableCleaningWarningLogsDirectoryForReddit,
    EnableCleaningWarningLogsDirectoryForTwitter,

    EnableProviders,
    EnableArxiv,
    EnableBiorxiv,
    EnableGithub,
    EnableHabr,
    EnabledMedrxiv,
    EnabledReddit,
    EnableTwitter,

    EnablePrints,
    EnablePrintsForProviders,
    EnablePrintsArxiv,
    EnablePrintsBiorxiv,
    EnablePrintsGithub,
    EnablePrintsHabr,
    EnablePrintsdMedrxiv,
    EnablePrintsdReddit,
    EnablePrintsTwitter,

    EnableWarningHighPrints,
    EnableWarningHighPrintsForProviders,
    EnableWarningHighPrintsForArxiv,
    EnableWarningHighPrintsForBiorxiv,
    EnableWarningHighPrintsForGithub,
    EnableWarningHighPrintsForHabr,
    EnableWarningHighPrintsForMedrxiv,
    EnableWarningHighPrintsForReddit,
    EnableWarningHighPrintsForTwitter,

    EnableWarninglowPrints,
    EnableWarninglowPrintsForProviders,
    EnableWarninglowPrintsForArxiv,
    EnableWarninglowPrintsForBiorxiv,
    EnableWarninglowPrintsForGithub,
    EnableWarninglowPrintsForHabr,
    EnableWarninglowPrintsForMedrxiv,
    EnableWarninglowPrintsForReddit,
    EnableWarninglowPrintsForTwitter,

    EnableSuccessPrints,
    EnableSuccessPrintsForProviders,
    EnableSuccessPrintsForArxiv,
    EnableSuccessPrintsForBiorxiv,
    EnableSuccessPrintsForGithub,
    EnableSuccessPrintsForHabr,
    EnableSuccessPrintsForMedrxiv,
    EnableSuccessPrintsForReddit,
    EnableSuccessPrintsForTwitter,

    EnablePartialSuccessPrints,
    EnablePartialSuccessPrintsForProviders,
    EnablePartialSuccessPrintsForArxiv,
    EnablePartialSuccessPrintsForBiorxiv,
    EnablePartialSuccessPrintsForGithub,
    EnablePartialSuccessPrintsForHabr,
    EnablePartialSuccessPrintsForMedrxiv,
    EnablePartialSuccessPrintsForReddit,
    EnablePartialSuccessPrintsForTwitter,

    EnableerrorPrints,
    EnableerrorPrintsForProviders,
    EnableerrorPrintsForArxiv,
    EnableerrorPrintsForBiorxiv,
    EnableerrorPrintsForGithub,
    EnableerrorPrintsForHabr,
    EnableerrorPrintsForMedrxiv,
    EnableerrorPrintsForReddit,
    EnableerrorPrintsForTwitter,

    EnableTimeMeasurementPrints,
    EnableTimeMeasurementPrintsForProviders,
    EnableTimeMeasurementForArxiv,
    EnableTimeMeasurementForBiorxiv,
    EnableTimeMeasurementForGithub,
    EnableTimeMeasurementForHabr,
    EnableTimeMeasurementForMedrxiv,
    EnableTimeMeasurementForReddit,
    EnableTimeMeasurementForTwitter,

    EnableinfoPrints,
    EnableinfoPrintsForProviders,
    EnableinfoForArxiv,
    EnableinfoForBiorxiv,
    EnableinfoForGithub,
    EnableinfoForHabr,
    EnableinfoForMedrxiv,
    EnableinfoForReddit,
    EnableinfoForTwitter,

    EnableproviderLinkslimit,
    EnablelinkslimitForArxiv,
    EnablelinkslimitForBiorxiv,
    EnablelinkslimitForGithub,
    EnablelinkslimitForHabr,
    EnablelinkslimitForMedrxiv,
    EnablelinkslimitForReddit,
    EnablelinkslimitForTwitter,

    EnableCommonProvidersLinkslimit,
}

impl EnvBoolVar {
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
}
