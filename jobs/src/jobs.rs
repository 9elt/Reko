use reko::Reko;

pub async fn update_old_users(reko: &Reko) {
    reko.update_old_users().await;
}

pub async fn update_airing_anime(reko: &Reko) {
    reko.update_airing_anime().await;
}

pub async fn request_missing_anime(reko: &Reko) {
    reko.request_missing_anime().await;
}
