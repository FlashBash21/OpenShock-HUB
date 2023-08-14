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
/// A single action to be preformed on a single Shocker
enum Action {
    Beep(Shocker_ID, time::Duration),
    Buzz(Shocker_ID, time::Duration, Intensity),
    Idle(Shocker_ID, time::Duration),
    Shock(Shocker_ID, time::Duration, Intensity)
}

/// A named series of actions to preform in order for a number of Shocker ids
struct Pattern {
    alias: String,
    timelines: Vec<Vec<Action>>
}

/// A set of restictions for valid shocker commands, and 
struct Permissions {
    max_beep: time::Duration,
    max_buzz: (time::Duration, Intensity),
    max_shock: (time::Duration, Intensity),
    patterns: HashSet<Pattern>
} 

/// A request for the host to send shome shock or other action
enum ShockerRequest {
    Action(Action),
    Pattern(Pattern)
} 

type Username = String;
type PublicKey = String;
/// Definitions for different client types and their relavent data
enum Client {
    Verified(Username, PublicKey, Permissions),
    Guest(Username, Permissions)
} 

/// This is the primary runtime for the OpenShock-Hub host server. All communications with remote clients will be managed through here.
struct OpenShockHubHost {
    router: axum::Router,
    client_profiles: HashSet<Client>,
    sessions: HashMap<Session, Client>
}

impl  OpenShockHubHost {

    fn new (client_profiles: HashSet<Client> ) -> Self {
        return Self {
            router: axum::Router::new(),
            sessions: HashMap::new(),
            client_profiles
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

    /// compare request against client permissions. If valid return success,
    /// else return request exceedes premission level
    async fn shock_request (self, session: Token, request: ShockerRequest) -> Result<String, StatusCode> {
        todo!()
    }
}


