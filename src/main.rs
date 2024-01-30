use reqwest::Error;
use scraper::{Html, Selector};
use regex::Regex;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let resp = reqwest::get("https://www.chinadaily.com.cn/a/202401/30/WS65b82bcca3104efcbdae88fc.html").await?;
    let body = resp.text().await?;

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("#Content").unwrap(); // 这里替换为你需要选择的 CSS 选择器
    let re = Regex::new(r"^\s*\n*\s*$").unwrap();

    for element in fragment.select(&selector) {
        let mut text = element.text().collect::<Vec<_>>();
        text.retain(|&x| !re.is_match(x));
        let tt = text.join(" ");
        
        let ret: Vec<&str> = tt.split(" ").collect();
        println!("{:?}", ret.len());
    }

    Ok(())
}