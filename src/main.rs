mod api;
mod db;
mod helper;

#[tokio::main]
async fn main() {
    let list = helper::list::get_detailed(format!("_nelt"), false).await;
    match list {
        Ok(_) => println!("OK!!!"),
        Err(e) => println!("{e}"),
    }
}
