use chrono::{Days, Utc};
use clients::myanimelist::MALClient;

#[tokio::main]
async fn main() {
    let mal = MALClient::new();

    if true {
        match mal.anime(54441).await {
            Ok(anime) => println!("{:#?}", anime),
            Err(error) => println!("{:#?}", error),
        }
    }

    if false {
        let user = String::from("_nelt");
        let updated_at = Utc::now().naive_utc().checked_sub_days(Days::new(5));

        match mal.list(user, updated_at).await {
            Ok(list) => println!("{:#?}", list),
            Err(error) => println!("{:#?}", error),
        }
    }
}
