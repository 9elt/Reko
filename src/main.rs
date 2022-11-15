mod api;
mod db;
mod helper;

#[tokio::main]
async fn main() {

    let anime = helper::anime::get(vec![33337]).await;
    println!("{:?}", anime);

    let list = helper::list::get(format!("_nelt"), false).await;
    match list {
        Ok(l) => println!("{:?}", l),
        Err(e) => println!("{e}") 
    }
}
