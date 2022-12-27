use crate::algorithm::{analysis, user};
use crate::helper;

/// ## Compute models for every user in the database
/// Returns an array `[successes, errors]`
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

pub async fn compute_normal_dist() -> Result<(), u16> {
    let normal_dist: analysis::NormalDist;
    match analysis::normal_distribution() {
        Ok(d) => normal_dist = d,
        Err(_) => return Err(500),
    }

    helper::save_normal_dist(normal_dist);
    Ok(())
}
