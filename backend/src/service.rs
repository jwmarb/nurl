use crate::structs::{ShortenedUrl, ShortenError, User};
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;


// take in the user, orig url, custom url (we randomize if not provided),
// expiry (if not provided then no expiration) 
pub fn createOrUpdateUrl (
    user: &User,
    original_url: &str,
    custom_url: &str,
    expiration_seconds: Option<u64>,
) -> Result<ShortenedUrl, ShortenError> {
    // make the url object

    // make a random and unique custom url if it is not proivded by user

    // get the time, make optional expiry, input the stuff to obj

    // then we can add to db
    Ok(())
}


// deletes a url (by id) for the user
pub fn deleteUrl(_user: &User, _id: &str) -> Result<(), ShortenError> {
    // rmeove the url in the db

    // check that it is owned by the user, then if so -> delete.s
    Ok(())
}

// returns a list of the shortened urls for a given user
pub fn listUrls(_user: &User) -> Result<Vec<ShortenedUrl>, ShortenError> {
    // query db and return the list
    Ok(Vec::new())
}

// redirect shortened url to the actual one
pub fn resolveUrl(custom_url: &str) -> Result<String, ShortenError> {
    // goto database, check expiry, return the redirect count and actual url, etc...
    Ok(format!("insert_actual_orig_url_here for: '{}'", custom_url))
}
