mod api;

#[tokio::main]
async fn main() {
    let anime = api::anime::get(50346).await;
    match anime {
        Ok(anime) => println!("{:?}", anime),
        Err(error) => println!("error! code: {}", error),
    }

    // let list = api::list::get("_nelt").await;
    // match list {
    //     Ok(list) => println!("{:?}", list.entries),
    //     Err(error) => println!("error! code: {}", error),
    // }
}
