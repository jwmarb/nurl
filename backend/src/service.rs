use crate::structs::{ShortenedUrl, User};
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

// take in the user, orig url, custom url (we randomize if not provided),
// expiry (if not provided then no expiration)
pub fn create_or_update_url(
    user: &User,
    original_url: &str,
    custom_url: Option<&str>,
    expiration_sec: Option<u64>,
) -> Result<ShortenedUrl, std::io::Error> {
    // TODO IMPLEMENT UPDATING FEATURE!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

    let cur_time = Utc::now();

    let id = Uuid::new_v4();

    // see if expiry provided, if so calulate the expiry date
    let expiry_date = if let Some(secs) = expiration_sec {
        Some(cur_time + Duration::seconds(secs as i64))
    } else {
        None
    };

    // Check if short url name provided. Otherwise generate a random one
    let final_custom_url = match custom_url {
        Some(url) if !url.is_empty() => {
            // TODO: make sure it is a valid one (no slashes in it, does not equal exactly auth)
            url.to_owned()
        }
        _ => {
            // TODO: NEED TO ENSURE UNIQUENESS, do some randomization and lookup to make sure it unique
            // use nanoid for this
            "abcde".to_string()
        }
    };

    // make the url object
    let short_url = ShortenedUrl {
        id: id,
        original_url: original_url.to_owned(),
        short_url: final_custom_url.to_owned(),
        expiry_date,
        created_at: cur_time,
        updated_at: cur_time,
        owner: user.id.clone(),
        redirects: 0,
    };

    // TODO: then we can add to db
    Ok(short_url)
}

// deletes a url (by id) for the user
pub fn delete_url(user: &User, id: &str) -> Result<(), std::io::Error> {
    // rmeove the url in the db

    // check that it is owned by the user, then if so -> delete.s
    Ok(())
}

// returns a list of the shortened urls for a given user
pub fn list_urls(user: &User) -> Result<Vec<ShortenedUrl>, std::io::Error> {
    // query db and return the list
    Ok(Vec::new())
}

// redirect shortened url to the actual one
pub fn resolve_url(custom_url: &str) -> Result<String, std::io::Error> {
    // goto database, check expiry, return the redirect count and actual url, etc...
    Ok(format!("insert_actual_orig_url_here for: '{}'", custom_url))
}
