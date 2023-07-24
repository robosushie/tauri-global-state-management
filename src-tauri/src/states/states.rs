use std::sync::Mutex;
use tauri::Manager;
use tauri::AppHandle;

use crate::autogen::constants::STATE_CHANGE_EVENT;
use crate::autogen::constants::STATE_SYNC_EVENT;
use crate::autogen::constants::GLOBAL_APP_STATE_MACRO;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum Value {
    Bool(bool),
    String(String),
    Number(i32),
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Payload {
  key: u32,
  value: Value,
}


pub struct GlobalAppState {
    count : Mutex<i32>,
    check: Mutex<bool>
}

pub fn set_default_state() -> GlobalAppState {
    GlobalAppState {
        count: Mutex::new(0),
        check: Mutex::new(false)
    }
}

fn emit_state_sync(payload: Payload, app_handle: AppHandle) {
    let state = app_handle.state::<GlobalAppState>();
    match GLOBAL_APP_STATE_MACRO::from(payload.key) {
        GLOBAL_APP_STATE_MACRO::COUNT => {
            let mut count = state.count.lock().unwrap();
            if let Value::Number(data) = payload.value {
                *count = data;
                app_handle.emit_all(STATE_SYNC_EVENT, Payload{key: payload.key, value:Value::Number(data)}).unwrap();
            }
            
        },
        GLOBAL_APP_STATE_MACRO::CHECK => {
            let mut check = state.check.lock().unwrap();
            if let Value::Bool(data) = payload.value {
                *check = data;
                app_handle.emit_all(STATE_SYNC_EVENT, Payload{key: payload.key, value:Value::Bool(data)}).unwrap();
            }
        }
    }  
}



pub fn subscribe_state_events(app_handle: tauri::AppHandle) {
    let app_handle_clone = app_handle.clone();
    let _id = app_handle.listen_global(STATE_CHANGE_EVENT, move |event| {
        if let Some(payload) = event.payload() {
            let data: Result<Payload, _> = serde_json::from_str(payload);
            match data {
                Ok(data) =>{
                    emit_state_sync(data, app_handle_clone.clone())
                }
                Err(e) => {
                    println!("Failed to deserialize payload: {}", e);
                }
            }
        }
    });
}

