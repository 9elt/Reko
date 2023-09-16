use chrono::{Days, Utc};
use clients::myanimelist::MALClient;
use clients::database::DBClient;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mal = MALClient::new();
    let db = DBClient::new();

    if true {
        let anime = match mal.anime(33337).await {
            Ok(anime) => {
                println!("{:#?}", anime);
                anime
            },
            Err(error) => panic!("{:#?}", error),
        };

        if db.insert_anime(anime) {
            let ani = db.get_anime(33337).expect("cannot parse db anime");
            println!("{:#?}", ani);
        }
        else {
            println!("failed insert");
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
