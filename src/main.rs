type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get(url : &str) -> Result<String> {
    let mut cache = static_http_cache::Cache::new(std::path::PathBuf::from("/var/tmp/cache"), reqwest::Client::new())?;
    let mut file = cache.get(reqwest::Url::parse(url)?)?;
    let mut string = String::with_capacity(file.metadata()?.len() as usize);
    use std::io::Read;
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn main() -> Result<()> {
    for url in ["https://readrust.net/all/feed.rss", "https://rust-gamedev.github.io/feed.xml"].iter() {
        match get(url)?.parse::<syndication::Feed>()? {
            syndication::Feed::Atom(feed) => for e in feed.entries() { println!("{}", e.links()[0].href()) },
            syndication::Feed::RSS(feed) => for e in feed.items() { if let Some(url) = e.link() { println!("{}", url); } },
        };
    }
    Ok(())
}
