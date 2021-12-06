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
pub enum EnvVar {
    GithubName,
    GithubToken,
    
    RedditUserAgent,
    RedditClientId,
    RedditClientSecret,
    RedditUsername,
    RedditPassword,
    
    MongoFirstHandleUrlPart,
    MongoSecondHandleUrlPart,
    MongoThirdHandleUrlPart,
    MongoFourthHandleUrlPart,
    MongoFifthHandleUrlPart,
    
    MongoLogin,
    MongoPassword,
    MongoIp,
    MongoPort,
    MongoParams,
    
    MongoProvidersLogsDbName,
    MongoProvidersLogsDbCollectionHandleSecondPart,
    MongoProvidersLogsDbCollectionDocumentFieldNameHandle,
    
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
    
    PostgresFirstHandleUrlPart,
    PostgresSecondHandleUrlPart,
    PostgresThirdHandleUrlPart,
    PostgresFourthHandleUrlPart,
    PostgresFifthHandleUrlPart,
    
    PostgresLogin,
    PostgresPassword,
    PostgresIp,
    PostgresPort,
    PostgresDb,
    
    WarningLogsDirectoryName,
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir,
    PathToProviderLinkPartsFolder,
    LogFileExtension,
    
    EnableWriteErrorLogsInLocalFolder,
    EnableWriteErrorLogsInLocalFolderForProvider,
    EnableWriteErrorLogsInLocalFolderForArxiv,
    EnableWriteErrorLogsInLocalFolderForBiorxiv,
    EnableWriteErrorLogsInLocalFolderForGithub,
    EnableWriteErrorLogsInLocalFolderForHabr,
    EnableWriteErrorLogsInLocalFolderForMedrxiv,
    EnableWriteErrorLogsInLocalFolderForReddit,
    EnableWriteErrorLogsInLocalFolderForTwitter,
    
    EnableCleaningWarningLogsDirectory,
    EnableCleaningWarningLogsDirectoryForProviders,
    EnableCleaningWarningLogsDirectoryForArxiv,
    EnableCleaningWarningLogsDirectoryForBiorxiv,
    EnableCleaningWarningLogsDirectoryForGithub,
    EnableCleaningWarningLogsDirectoryForHabr,
    EnableCleaningWarningLogsDirectoryForMedrxiv,
    EnableCleaningWarningLogsDirectoryForReddit,
    EnableCleaningWarningLogsDirectoryForTwitter,
    
    StartingCheckLink,
    ArxivCheckLink,
    BiorxivCheckLink,
    GithubCheckLink,
    HabrCheckLink,
    MedrxivCheckLink,
    RedditCheckLink,
    TwitterCheckLink,
    
    EnableProviders,
    EnableArxiv,
    EnableBiorxiv,
    EnableGithub,
    EnableHabr,
    EnableMedrxiv,
    EnableReddit,
    EnableTwitter,
    
    EnablePrints,
    EnablePrintsForProviders,
    EnablePrintsArxiv,
    EnablePrintsBiorxiv,
    EnablePrintsGithub,
    EnablePrintsHabr,
    EnablePrintsMedrxiv,
    EnablePrintsReddit,
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
    
    EnableWarningLowPrints,
    EnableWarningLowPrintsForProviders,
    EnableWarningLowPrintsForArxiv,
    EnableWarningLowPrintsForBiorxiv,
    EnableWarningLowPrintsForGithub,
    EnableWarningLowPrintsForHabr,
    EnableWarningLowPrintsForMedrxiv,
    EnableWarningLowPrintsForReddit,
    EnableWarningLowPrintsForTwitter,
    
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
    
    EnableErrorPrints,
    EnableErrorPrintsForProviders,
    EnableErrorPrintsForArxiv,
    EnableErrorPrintsForBiorxiv,
    EnableErrorPrintsForGithub,
    EnableErrorPrintsForHabr,
    EnableErrorPrintsForMedrxiv,
    EnableErrorPrintsForReddit,
    EnableErrorPrintsForTwitter,
    
    EnableTimeMeasurementPrints,
    EnableTimeMeasurementPrintsForProviders,
    EnableTimeMeasurementForArxiv,
    EnableTimeMeasurementForBiorxiv,
    EnableTimeMeasurementForGithub,
    EnableTimeMeasurementForHabr,
    EnableTimeMeasurementForMedrxiv,
    EnableTimeMeasurementForReddit,
    EnableTimeMeasurementForTwitter,
    
    EnableInfoPrints,
    EnableInfoPrintsForProviders,
    EnableInfoForArxiv,
    EnableInfoForBiorxiv,
    EnableInfoForGithub,
    EnableInfoForHabr,
    EnableInfoForMedrxiv,
    EnableInfoForReddit,
    EnableInfoForTwitter,
    
    EnableLinksLimit,
    EnableLinksLimitForProviders,
    EnableLinksLimitForArxiv,
    EnableLinksLimitForBiorxiv,
    EnableLinksLimitForGithub,
    EnableLinksLimitForHabr,
    EnableLinksLimitForMedrxiv,
    EnableLinksLimitForReddit,
    EnableLinksLimitForTwitter,
    
    EnableCommonProvidersLinksLimit,
    CommonProvidersLinksLimit,
    LinksLimitForArxiv,
    LinksLimitForBiorxiv,
    LinksLimitForGithub,
    LinksLimitForHabr,
    LinksLimitForMedrxiv,
    LinksLimitForReddit,
    LinksLimitForTwitter,
    
    ErrorRed,
    ErrorGreen,
    ErrorBlue,
    WarningHighRed,
    WarningHighGreen,
    WarningHighBlue,
    WarningLowRed,
    WarningLowGreen,
    WarningLowBlue,
    SuccessRed,
    SuccessGreen,
    SuccessBlue,
    PartialSuccessRed,
    PartialSuccessGreen,
    PartialSuccessBlue,
    CleaningRed,
    CleaningGreen,
    CleaningBlue,
    TimeMeasurementRed,
    TimeMeasurementGreen,
    TimeMeasurementBlue,
    InfoRed,
    InfoGreen,
    InfoBlue,
}

impl EnvVar {
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
}
