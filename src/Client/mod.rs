use std::collections::HashMap;

use reqwest::blocking;
use serde_json::Value;

pub enum State {
    Error,
    Running,
    Finished,
}

pub struct Client {
    client: blocking::Client,
    host: String,
}

impl Client {
    pub fn new(host: &str, token: &str) -> Client {
        Client {
            client: blocking::Client::builder()
                .user_agent("zfs-replicator")
                .default_headers(reqwest::header::HeaderMap::from_iter(vec![
                    (
                        reqwest::header::AUTHORIZATION,
                        format!("Basic {}", token).parse().unwrap(),
                    ),
                    (reqwest::header::ACCEPT, "application/json".parse().unwrap()),
                ]))
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            host: host.to_string(),
        }
    }

    pub fn List_replication_tasks(&self) -> Result<HashMap<String, usize>, String> {
        let response = self
            .client
            .get(&format!(
                "https://{}/api/v2.0/replication?count=false",
                self.host
            ))
            .send()
            .map_err(|e| format!("failed to list replication tasks: {}", e))?;

        let json: Value = response
            .json()
            .map_err(|e| format!("failed to parse json: {}", e))?;

        let mut tasks = HashMap::new();
        for task in json.as_array().unwrap() {
            tasks.insert(
                task["name"].as_str().unwrap().to_string(),
                task["id"].as_u64().unwrap() as usize,
            );
        }

        Ok(tasks)
    }

    pub fn Send_replication_request(
        &self,
        task_id: usize
    ) -> Result<(), String> {
        let response = self
            .client
            .post(&format!(
                "https://{}/api/v2.0/replication/id/{}/run",
                self.host, task_id
            ))
            .send()
            .map_err(|e| format!("failed to send replication request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "failed to send replication request: {}",
                response.status()
            ));
        }

        Ok(())
    }

    pub fn Get_replication_task_state(
        &self,
        task_id: usize
    ) -> Result<State, String> {
        let response = self
            .client
            .get(&format!(
                "https://{}/api/v2.0/replication/id/{}",
                self.host, task_id
            ))
            .send()
            .map_err(|e| format!("failed to get replication task status: {}", e))?;

        let json: Value = response
            .json()
            .map_err(|e| format!("failed to parse json: {}", e))?;

        let state = json["state"]["state"].as_str().ok_or("Failed to parse state")?; // "RUNNING", "ERROR", "SUCCESS

        match state {
            "ERROR" => Ok(State::Error),
            "RUNNING" => Ok(State::Running),
            "FINISHED" => Ok(State::Finished),
            _ => Err(format!("unknown state: {}", state)),
        }
    }
}
