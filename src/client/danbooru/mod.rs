use reqwest::Client;

use crate::model::danbooru::*;

/// Client that sends requests to the Danbooru API to retrieve the data.
#[allow(dead_code)]
pub struct DanbooruClient {
    client: Client,
    key: Option<String>
}

impl DanbooruClient {
    pub fn builder() -> DanbooruClientBuilder {
        DanbooruClientBuilder::default()
    }
}

/// Builder for [`DanbooruClient`]
#[derive(Default)]
pub struct DanbooruClientBuilder {
    client: Client,
    key: Option<String>,
    user: Option<String>,
    tags: Vec<String>,
    limit: u32,
}

impl DanbooruClientBuilder {
    pub fn new() -> DanbooruClientBuilder {
        DanbooruClientBuilder {
            client: Client::new(),
            key: None,
            user: None,
            tags: vec![],
            limit: 100,
        }
    }
    /// Set the API key and User for the requests (optional)
    pub fn set_credentials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }

    /// Add a tag to the query
    pub fn tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Add a [`DanbooruRating`] to the query
    pub fn rating(mut self, rating: DanbooruRating) -> Self {
        self.tags.push(format!("rating:{}", rating));
        self
    }

    /// Set how many posts you want to retrieve (100 is the default and maximum)
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Retrieves the posts in a random order
    pub fn random(mut self, random: bool) -> Self {
        if random {
            self.tags.push("order:random".to_string());
        }
        self
    }

    /// Add a [`DanbooruSort`] to the query
    pub fn sort(mut self, order: DanbooruSort) -> Self {
        self.tags.push(format!("order:{}", order));
        self
    }

    /// Blacklist a tag from the query
    pub fn blacklist_tag(mut self, tag: String) -> Self {
        self.tags.push(format!("-{tag}"));
        self
    }

    /// Directly get a post by its unique Id
    pub async fn get_by_id(&self, id: u32) -> Result<DanbooruPost, reqwest::Error> {
        let response = self.client
            .get(format!("https://danbooru.donmai.us/posts/{id}.json"))
            .send()
            .await?
            .json::<DanbooruPost>()
            .await?;

        Ok(response)
    }

    /// Pack the [`DanbooruClientBuilder`] and sent the request to the API to retrieve the posts
    pub async fn get(&self) -> Result<Vec<DanbooruPost>, reqwest::Error> {
        let tag_string = self.tags.join(" ");
        let response = self.client
            .get("https://danbooru.donmai.us/posts.json")
            .query(&[
                ("limit", self.limit.to_string().as_str()),
                ("tags", &tag_string),
            ])
            .send()
            .await?
            .json::<Vec<DanbooruPost>>()
            .await?;

        println!("{response:#?}");

        Ok(response)
    }
}
