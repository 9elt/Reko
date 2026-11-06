use crate::reko::Reko;
use chrono::Local;
use croner::Cron;
use std::future::Future;
use std::str::FromStr;

pub fn spawn(reko: Reko) {
    let r = reko.clone();
    schedule("0,15,30,45 4 * * *", "update_old_users", move || {
        let r = r.clone();
        async move { r.update_old_users(progress).await }
    });

    let r = reko.clone();
    schedule("0 2 * * MON", "request_missing_anime", move || {
        let r = r.clone();
        async move { r.request_missing_anime(progress).await }
    });

    schedule("0 2 * * TUE", "update_airing_anime", move || {
        let r = reko.clone();
        async move { r.update_airing_anime(progress).await }
    });
}

fn schedule<F, Fut>(cron: &str, name: &'static str, job: F)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    let cron = Cron::from_str(cron).expect("invalid cron");

    tokio::spawn(async move {
        loop {
            let now = Local::now();
            let next = cron.find_next_occurrence(&now, false).unwrap();
            tokio::time::sleep((next - now).to_std().unwrap_or_default()).await;

            println!("job {name}: start");
            job().await;
            println!("job {name}: done");
        }
    });
}

fn progress(curr: u32, tot: u32) {
    if curr == tot {
        println!("{curr}/{tot}");
    }
}
