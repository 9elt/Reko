use crate::helper;

use crate::algorithm::fucker;

pub async fn compute_all_models() -> Result<[i32; 2], u16> {

    let usernames: Vec<String>;

    match helper::get_all_usernames() {
        Ok(res) => usernames = res,
        Err(_) => return Err(500)
    }

    let mut ok_err = [0, 0];

    println!("start...");

    for user in usernames.iter() {
        match fucker::stats::stats_model(user.to_owned(), true, true).await {
            Ok(_) => ok_err[0] += 1,
            Err(_) => ok_err[1] += 1,
        };
        print!("\r \x1b[34m\x1b[1m{}\x1b[0m / {} -> \x1b[32m\x1b[1m{} OK \x1b[31m{} ERR\x1b[0m", ok_err[0] + ok_err[1], usernames.len(), ok_err[0], ok_err[1]);
    }

    Ok(ok_err)
}
