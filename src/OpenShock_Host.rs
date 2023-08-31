use axum::{
    response::{Html, Response},
    routing::get,
    Router,
};

use oauth2;

use std::{time, string};
use std::string::String;
use std::collections::{
    hash_map::HashMap,
    hash_set::HashSet
};

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
            router: axum::Router::new()
            .route("/api/", get(Self::auth)),
            sessions: HashMap::new(),
            client_profiles
        };
    }

    async fn app(self) {
        
    } 
}


