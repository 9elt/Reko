mod fetch;

#[tokio::main]
async fn main() {
    let list = fetch::fun::get_detailed_list(format!("acha_uwu"), false).await;
    match list {
        Ok(l) => println!("{:?} OK!!!", l[0]),
        Err(e) => println!("{e}"),
    }
}
