use axum::{self, http::StatusCode};
use oauth2;

use std::time;
use std::string::String;
use std::collections::{
    hash_map::HashMap,
    hash_set::HashSet
};

type Token = String;
struct Session {
    token: Token,
    expiration: Option<time::SystemTime>
}

type Intensity = f32; 
type Shocker_ID = usize; 
enum Action {
    Beep(Shocker_ID, time::Duration),
    Buzz(Shocker_ID, time::Duration, Intensity),
    Idle(Shocker_ID, time::Duration),
    Shock(Shocker_ID, time::Duration, Intensity)
}

struct Pattern {
    alias: String,
    timeline: Vec<Action>
}

struct Permissions {
    max_beep: time::Duration,
    max_buzz: (time::Duration, Intensity),
    max_shock: (time::Duration, Intensity),
    patterns: HashSet<Pattern>
} 

type Username = String;
type PublicKey = String;
enum Client {
    Root(Username, PublicKey),
    Verified(Username, PublicKey, Permissions),
    Guest(Username, Permissions)
} 

/// This is the primary runtime for the OpenShock-Hub host server. All communications with remote clients will be managed through here.
struct OpenShockHubHost {
    router: axum::Router,
    clients: HashSet<Client>,
    sessions: HashMap<Session, Client>
}

impl  OpenShockHubHost {

    fn new (clients: HashSet<Client> ) -> Self {
        return Self {
            router: axum::Router::new(),
            sessions: HashMap::new(),
            clients
        };
    }

    // Handlers 

    /// Generate a new session token and add it to sessions. 
    /// Will automatically check clients list for username. 
    /// If no username found creates guest client session and returns success. 
    /// Else will try to validate client by handshake using public key data.
    /// If client gives correct response Create session with stored client data and permissions.
    /// Else give err "failed verification for username if attemtping to use a guest client change usernames, otherwise ensure that the host has entered the correct public key"   
    async fn new_session (self, username: Username) -> Result<String, StatusCode> {
        todo!()
    }

    async fn close_session (self, session: Token) -> Result<String, StatusCode> {
        todo!()
    }

    async fn permissions_request (self, session: Token) -> Result<Session, StatusCode> {
        todo!()
    }

    async fn cmd_request (self, session: Token, ) -> Result<String, StatusCode> {
        todo!()
    }
}


