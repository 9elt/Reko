use reko::Reko;

pub async fn update_old_users(reko: &Reko) {
    println!("updating old users");
    reko.update_old_users(progress).await;
}

pub async fn update_airing_anime(reko: &Reko) {
    println!("updating airing anime");
    reko.update_airing_anime(progress).await;
}

pub async fn request_missing_anime(reko: &Reko) {
    println!("requesting missing anime");
    reko.request_missing_anime(progress).await;
}

fn progress(curr: u32, tot: u32) {
    print!("{}/{} ", curr, tot);
}
