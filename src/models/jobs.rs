use crate::algorithm::{analysis, user};
use crate::helper;

/// ### Compute models for every user in the database
pub async fn compute_all_models() -> Result<[i32; 2], u16> {
    let usernames: Vec<String>;
    match helper::get_all_usernames() {
        Ok(res) => usernames = res,
        Err(_) => return Err(500),
    }

    let mut status = [0, 0];
    println!("compute all models started...");

    for user in usernames.iter() {
        let user_list = match helper::get_detailed_list(user, false, false).await {
            Ok(val) => val,
            Err(_) => {
                status[1] += 1;
                continue;
            }
        };

        helper::save_user_model(&user, &user::stats::stats_model(user_list));
        status[0] += 1;

        print!(
            "\r\x1b[34m\x1b[1m{}\x1b[0m / {} -> \x1b[32m\x1b[1m{} OK \x1b[31m{} ERR\x1b[0m",
            status[0] + status[1],
            usernames.len(),
            status[0],
            status[1]
        );
    }

    Ok(status)
}

/// ### Compute normal distribution
pub async fn compute_normal_dist() -> Result<(), u16> {
    let normal_dist: analysis::NormalDist;
    match analysis::normal_distribution() {
        Ok(d) => normal_dist = d,
        Err(_) => return Err(500),
    }

    helper::save_normal_dist(normal_dist);
    Ok(())
}

/// ### Update 100 users that are older than 5 days
pub async fn update_old_users() -> Result<[i32; 2], u16> {
    let usernames = match helper::get_old_usernames() {
        Ok(res) => res,
        Err(_) => return Err(500),
    };

    let mut ok_err = [0; 2];

    for user in usernames {
        let list = match helper::get_detailed_list(&user, true, false).await {
            Ok(val) => val,
            Err(_) => {
                ok_err[1] += 1;
                continue;
            }
        };

        ok_err[0] += 1;

        println!(
            "\r\x1b[34m\x1b[1m{}\x1b[0m -> \x1b[32m\x1b[1m{} OK \x1b[31m{} ERR\x1b[0m",
            ok_err.iter().sum::<i32>(),
            ok_err[0],
            ok_err[1]
        );

        let stats_model = user::stats::stats_model(list);
        helper::save_user_model(&user, &stats_model);

        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(ok_err)
}

/// ### Update airing and not aired yet anime
pub async fn update_airing_anime() -> Result<[i32; 2], u16> {
    let anime = match helper::get_airing_anime() {
        Ok(res) => res,
        Err(_) => return Err(500),
    };

    let tot = anime.len();
    let mut ok_err = [0; 2];

    for id in anime.iter() {
        match helper::mal_api::anime::get(id).await {
            Ok(res) => {
                match helper::update_anime(res.serialize()) {
                    Ok(_) => ok_err[0] += 1,
                    Err(_) => ok_err[1] += 1
                }
            },
            Err(_) => {
                ok_err[1] += 1;
            }
        };

        println!(
            "\r\x1b[34m\x1b[1m{}\x1b[0m / {} -> \x1b[32m\x1b[1m{} OK \x1b[31m{} ERR\x1b[0m",
            ok_err.iter().sum::<i32>(),
            tot,
            ok_err[0],
            ok_err[1]
        );

        std::thread::sleep(std::time::Duration::from_millis(300));        
    }

    Ok(ok_err)
}
