use std::env::var;

pub async fn tweet(tweet_str: &str) {
    let base = "twitter_kiararaten_bot";
    let api_key = var(format!("{}_{}",base,"api_key")).unwrap();
    let api_secret_key = var(format!("{}_{}",base,"api_secret_key")).unwrap();
    let access_token_secret = var(format!("{}_{}",base,"access_token_secret")).unwrap();
    let access_token = var(format!("{}_{}",base,"access_token")).unwrap();

    let builder = kuon::TwitterAPI::builder()
        .access_token(access_token)
        .access_token_secret(access_token_secret)
        .api_key(api_key)
        .api_secret_key(api_secret_key);
    let api = builder.build().await.unwrap();
    api.tweet().status(tweet_str).send().await.unwrap();
}
