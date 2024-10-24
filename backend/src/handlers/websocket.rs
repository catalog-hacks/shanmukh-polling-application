use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use tokio::sync::broadcast::{self, Sender};
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Clone, Serialize)]
pub struct VoteResult {
    pub _id: String,
    pub count: i32,
}

#[derive(Clone, Serialize)]
pub enum PollUpdate {
    VoteUpdate { poll_id: String, results: Vec<VoteResult> },
    StatusUpdate { poll_id: String, is_active: bool },
    Reset { poll_id: String },
}


// Using a simple counter for unique session IDs
static SESSION_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

// Modified structure to store Session with a unique ID
struct SessionWrapper {
    id: usize,
    _session: Session,
}

static POLL_UPDATES: Lazy<(Sender<PollUpdate>, RwLock<HashMap<String, Vec<SessionWrapper>>>)> = Lazy::new(|| {
    let (tx, _) = broadcast::channel(100);
    (tx, RwLock::new(HashMap::new()))
});

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    poll_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;
    
    let session_id = SESSION_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let session_wrapper = SessionWrapper {
        id: session_id,
        _session: session.clone(),
    };
    
    // Store the session for this poll
    let mut connections = POLL_UPDATES.1.write().await;
    connections
        .entry(poll_id.to_string())
        .or_default()
        .push(session_wrapper);

    // Spawn WebSocket handler task
    actix_web::rt::spawn(ws_client(
        session,
        msg_stream,
        poll_id.to_string(),
        session_id,
        POLL_UPDATES.0.subscribe(),
    ));

    Ok(response)
}

async fn ws_client(
    mut session: Session,
    mut msg_stream: MessageStream,
    poll_id: String,
    session_id: usize,
    mut broadcast_rx: broadcast::Receiver<PollUpdate>,
) {
    let mut closed = false;

    while !closed {
        tokio::select! {
            Some(msg) = msg_stream.next() => {
                match msg {
                    Ok(Message::Close(_)) => {
                        closed = true;
                    }
                    Ok(Message::Ping(bytes)) => {
                        if session.pong(&bytes).await.is_err() {
                            closed = true;
                        }
                    }
                    Err(_) => {
                        closed = true;
                    }
                    _ => {}
                }
            }
            Ok(update) = broadcast_rx.recv() => {
                match &update {
                    PollUpdate::VoteUpdate { poll_id: update_poll_id, .. } |
                    PollUpdate::StatusUpdate { poll_id: update_poll_id, .. } |
                    PollUpdate::Reset { poll_id: update_poll_id } => {
                        if *update_poll_id == poll_id {
                            if let Ok(json) = serde_json::to_string(&update) {
                                if session.text(json).await.is_err() {
                                    closed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Clean up connection
    let mut connections = POLL_UPDATES.1.write().await;
    if let Some(poll_sessions) = connections.get_mut(&poll_id) {
        poll_sessions.retain(|wrapper| wrapper.id != session_id);
        if poll_sessions.is_empty() {
            connections.remove(&poll_id);
        }
    }
}

pub async fn broadcast_poll_update(update: PollUpdate) {
    let _ = POLL_UPDATES.0.send(update);
}