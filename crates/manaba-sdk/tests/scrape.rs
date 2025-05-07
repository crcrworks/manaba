use manaba_sdk::error::Result;
use manaba_sdk::{Client, Cookie};

pub const COOKIE_DOMAIN: &str = "ct.ritsumei.ac.jp";
pub const BASE_URL: &str = "https://ct.ritsumei.ac.jp/ct/";

#[tokio::test]
async fn get_courses_test() -> Result<()> {
    let cookie = Cookie::load(COOKIE_DOMAIN)?;

    let client = Client::new(BASE_URL, &cookie).await?;
    let _courses = client.get_courses().await?;

    Ok(())
}

#[tokio::test]
async fn get_exams_test() -> Result<()> {
    let cookie = Cookie::load(COOKIE_DOMAIN)?;

    let client = Client::new(BASE_URL, &cookie).await?;
    let courses = client.get_courses().await?;

    for course in courses {
        let _exams = client.get_exams(&course).await?;
    }

    Ok(())
}

#[tokio::test]
async fn get_reports_test() -> Result<()> {
    let cookie = Cookie::load(COOKIE_DOMAIN)?;

    let client = Client::new(BASE_URL, &cookie).await?;
    let courses = client.get_courses().await?;

    for course in courses {
        let _reports = client.get_reports(&course).await?;
    }

    Ok(())
}
