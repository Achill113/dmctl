use std::borrow::Cow;

use crate::infrastructure::muna::results::GetNamesResult;

pub struct MunaClient {
}

impl MunaClient {
    const BASE_URL: Cow<'static, str> = Cow::Borrowed("https://muna.ironarachne.com");

    pub async fn get_names(race: String, count: i16, gender: String) -> Result<GetNamesResult, Box<dyn std::error::Error>> {
        let url = format!("{}/{}?count={}&nameType={}", Self::BASE_URL, str::to_lowercase(&race), count, str::to_lowercase(&gender));

        let response = reqwest::get(url).await?.json::<GetNamesResult>().await?;

        Ok(response)
    }
}

