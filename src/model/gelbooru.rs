//! Models for Gelbooru
use core::fmt;

use serde::{Serialize, Deserialize};

/// Individual post from [`GelbooruResponse`]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GelbooruPost {
    /// The ID of the post
    pub id: u32,
    /// Datestamp of the post's creating date
    pub created_at: String,
    /// Post's score
    pub score: u32,
    /// Post's image width
    pub width: u32,
    /// Post's image height
    pub height: u32,
    /// Post's image md5
    pub md5: String,
    /// Post's image file url
    pub file_url: String,
    /// Post's tags
    pub tags: String,
    /// Post's image name (with extension)
    pub image: String,
    /// Post's image source
    pub source: String,
    /// Post's rating
    pub rating: GelbooruRating
}

/// Gelbooru's API response with a list a posts
#[derive(Serialize, Deserialize, Debug)]
pub struct GelbooruResponse {
    #[serde(rename = "post")]
    pub posts: Vec<GelbooruPost>
}

/// Post's rating. Check the [Gelbooru's ratings wiki](https://gelbooru.com/index.php?page=help&topic=rating)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GelbooruRating {
    Explicit,
    Questionable,
    Safe,
    Sensitive,
    General
}

impl fmt::Display for GelbooruRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lovercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lovercase_tag}")
    }
}

#[derive(Debug, Clone)]
pub enum GelbooruSort {
    Id,
    Score,
    Rating,
    User,
    Height,
    Width,
    Source,
    Updated
}

impl fmt::Display for GelbooruSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lovercase_tag = format!("{:?}", self).to_lowercase();
        write!(f, "{lovercase_tag}")
    }
}
