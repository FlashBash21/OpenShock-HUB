use axum::{self, http::StatusCode};
use oauth2;

use std::time;
use std::string::String;
use std::collections::{
    hash_map::HashMap,
    hash_set::HashSet
};

struct Session {
    token: String,
    expiration: Option<time::SystemTime>
}

struct Client {
    username: String,
    public_key: Option<String>  
} 

/// This is the primary runtime for the OpenShock-Hub host server. All communications with remote clients will be managed through here.
struct OpenShockHubHost {
    router: axum::Router,
    sessions: HashMap<Session, Client>
}

impl  OpenShockHubHost {

    fn new () -> Self {
        return Self {
            router: axum::Router::new(),
            sessions: HashMap::new()
        };
    }

    // Handlers 
    async fn new_session (self, ) -> Result<String, StatusCode> {

    }
    
}

