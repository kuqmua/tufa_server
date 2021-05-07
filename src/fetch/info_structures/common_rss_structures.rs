#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonRssPostStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<CommonRssPost>,
}

impl CommonRssPostStruct {
    pub fn new() -> Self {
        CommonRssPostStruct {
            items: Vec::<CommonRssPost>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CommonRssPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
    //meta
    pub provider_name: String,
    //meta

    //arxiv specific
    //nothing here yet
    //arxiv specific

    //biorxiv specific
    pub biorxiv_date: Option<String>,
    pub biorxiv_identifier: Option<String>,
    pub biorxiv_publisher: Option<String>,
    pub biorxiv_publication_date: Option<String>,
    //biorxiv specific

    //habr specific
    pub habr_guid: Option<String>,
    pub habr_pub_date: Option<String>,
    pub habr_category: Option<Vec<String>>,
    //habr specific

    //medrxiv specific
    pub medrxiv_date: Option<String>,
    pub medrxiv_identifier: Option<String>,
    pub medrxiv_publisher: Option<String>,
    pub medrxiv_publication_date: Option<String>,
    //medrxiv specific

    //reddit specific
    // pub reddit_selftext: String,//into description feild
    pub reddit_url_overridden_by_dest: Option<String>,
    pub reddit_subreddit: Option<String>,
    pub reddit_id: Option<String>,
    pub reddit_author_fullname: Option<String>,
    pub reddit_domain: Option<String>,
    pub reddit_permalink: Option<String>,
    pub reddit_thumbnail: Option<String>,
    pub reddit_created_utc: Option<f64>,
    pub reddit_ups: Option<f64>,
    pub reddit_score: Option<f64>,
    pub reddit_num_comments: Option<u64>,
    pub reddit_over_18: Option<bool>,
    pub reddit_quarantine: Option<bool>,
    pub reddit_is_self: Option<bool>,
    pub reddit_saved: Option<bool>,
    pub reddit_url: Option<String>,
    pub reddit_hidden: Option<bool>,
    pub reddit_downs: Option<f64>,
    pub reddit_name: Option<String>,
    pub reddit_upvote_ratio: Option<f64>,
    pub reddit_total_awards_received: Option<f64>,
    pub reddit_is_original_content: Option<bool>,
    pub reddit_is_reddit_media_domain: Option<bool>,
    pub reddit_is_meta: Option<bool>,
    pub reddit_created: Option<f64>,
    pub reddit_allow_live_comments: Option<bool>,
    pub reddit_archived: Option<bool>,
    pub reddit_is_crosspostable: Option<bool>,
    pub reddit_pinned: Option<bool>,
    pub reddit_media_only: Option<bool>,
    pub reddit_spoiler: Option<bool>,
    pub reddit_locked: Option<bool>,
    pub reddit_visited: Option<bool>,
    pub reddit_subreddit_id: Option<String>,
    pub reddit_send_replies: Option<bool>,
    pub reddit_stickied: Option<bool>,
    pub reddit_subreddit_subscribers: Option<f64>,
    pub reddit_is_video: Option<bool>,
    //reddit specific

    //twitter specific
    pub twitter_pub_date: Option<String>,
    pub twitter_guid: Option<String>,
    //twitter specific
}
impl CommonRssPost {
    pub fn initialize_with_params(
        title: String,
        link: String,
        description: String,
        creator: String,
        //meta
        provider_name: String,
        //meta

        //arxiv specific
        //nothing here yet
        //arxiv specific

        //biorxiv specific
        biorxiv_date: Option<String>,
        biorxiv_identifier: Option<String>,
        biorxiv_publisher: Option<String>,
        biorxiv_publication_date: Option<String>,
        //biorxiv specific

        //habr specific
        habr_guid: Option<String>,
        habr_pub_date: Option<String>,
        habr_category: Option<Vec<String>>,
        //habr specific

        //medrxiv specific
        medrxiv_date: Option<String>,
        medrxiv_identifier: Option<String>,
        medrxiv_publisher: Option<String>,
        medrxiv_publication_date: Option<String>,
        //medrxiv specific

        //reddit specific
        // reddit_selftext: String,//into description feild
        reddit_url_overridden_by_dest: Option<String>,
        reddit_subreddit: Option<String>,
        reddit_id: Option<String>,
        reddit_author_fullname: Option<String>,
        reddit_domain: Option<String>, //host site
        reddit_permalink: Option<String>,
        reddit_thumbnail: Option<String>, //image link todo
        reddit_created_utc: Option<f64>,
        reddit_ups: Option<f64>,
        reddit_score: Option<f64>, //difference between score от ups?
        reddit_num_comments: Option<u64>,
        reddit_over_18: Option<bool>,
        reddit_quarantine: Option<bool>,
        reddit_is_self: Option<bool>, //can be usefull in client app
        reddit_saved: Option<bool>,
        reddit_url: Option<String>,
        reddit_hidden: Option<bool>,
        reddit_downs: Option<f64>,
        reddit_name: Option<String>,
        reddit_upvote_ratio: Option<f64>,
        reddit_total_awards_received: Option<f64>,
        reddit_is_original_content: Option<bool>,
        reddit_is_reddit_media_domain: Option<bool>,
        reddit_is_meta: Option<bool>,
        reddit_created: Option<f64>, //time
        reddit_allow_live_comments: Option<bool>,
        reddit_archived: Option<bool>,
        reddit_is_crosspostable: Option<bool>,
        reddit_pinned: Option<bool>,
        reddit_media_only: Option<bool>,
        reddit_spoiler: Option<bool>,
        reddit_locked: Option<bool>,
        reddit_visited: Option<bool>,
        reddit_subreddit_id: Option<String>,
        reddit_send_replies: Option<bool>,
        reddit_stickied: Option<bool>,
        reddit_subreddit_subscribers: Option<f64>,
        reddit_is_video: Option<bool>,
        //reddit specific

        //twitter specific
        twitter_pub_date: Option<String>,
        twitter_guid: Option<String>,
        //twitter specific
    ) -> Self {
        CommonRssPost {
            title,
            link,
            description,
            creator,
            //meta
            provider_name,
            //meta

            //arxiv specific
            //nothing here yet
            //arxiv specific

            //biorxiv specific
            biorxiv_date,
            biorxiv_identifier,
            biorxiv_publisher,
            biorxiv_publication_date,
            //biorxiv specific

            //habr specific
            habr_guid,
            habr_pub_date,
            habr_category,
            //habr specific

            //medrxiv specific
            medrxiv_date,
            medrxiv_identifier,
            medrxiv_publisher,
            medrxiv_publication_date,
            //medrxiv specific

            //reddit specific
            // selftext: String,//into description feild
            reddit_url_overridden_by_dest,
            reddit_subreddit,
            reddit_id,
            reddit_author_fullname,
            reddit_domain,
            reddit_permalink,
            reddit_thumbnail,
            reddit_created_utc,
            reddit_ups,
            reddit_score,
            reddit_num_comments,
            reddit_over_18,
            reddit_quarantine,
            reddit_is_self,
            reddit_saved,
            reddit_url,
            reddit_hidden,
            reddit_downs,
            reddit_name,
            reddit_upvote_ratio,
            reddit_total_awards_received,
            reddit_is_original_content,
            reddit_is_reddit_media_domain,
            reddit_is_meta,
            reddit_created,
            reddit_allow_live_comments,
            reddit_archived,
            reddit_is_crosspostable,
            reddit_pinned,
            reddit_media_only,
            reddit_spoiler,
            reddit_locked,
            reddit_visited,
            reddit_subreddit_id,
            reddit_send_replies,
            reddit_stickied,
            reddit_subreddit_subscribers,
            reddit_is_video,
            //reddit specific

            //twitter specific
            twitter_pub_date,
            twitter_guid,
            //twitter specific
        }
    }
}
