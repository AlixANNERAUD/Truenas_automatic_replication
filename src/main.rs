#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{collections::HashMap, env::var, process::Command};

use log::info;

mod Client;

fn Import_dataset(name: &str) -> Result<(), String> {
    Command::new("sudo")
        .arg("zpool")
        .arg("import")
        .arg(name)
        .output()
        .map_err(|e| format!("{}", e))?;

    Ok(())
}

fn Export_dataset(name: &str) -> Result<(), String> {
    Command::new("sudo")
        .arg("zpool")
        .arg("export")
        .arg(name)
        .output()
        .map_err(|e| format!("{}", e))?;

    Ok(())
}

fn Shutdown_disk(name: &str) -> Result<(), String> {
    Command::new("udisksctl")
        .arg("power-off")
        .arg("-b")
        .arg(name)
        .output()
        .map_err(|e| format!("{}", e))?;

    Ok(())
}

fn main() {
    // - Initialize the logger
    env_logger::init();

    // - Read environment variables
    let host = var("TRUENAS_SCALE_HOST").unwrap();
    let token = var("TRUENAS_SCALE_TOKEN").unwrap();
    let tasks_raw = var("TRUENAS_SCALE_TASKS").unwrap();
    let tasks = tasks_raw.split(":").collect::<Vec<_>>();
    let datasets_raw = var("LOCAL_DATASETS").unwrap();
    let datasets = datasets_raw.split(":").collect::<Vec<_>>();
    let disks_raw = var("LOCAL_DISKS").unwrap();
    let disks = disks_raw.split(":").collect::<Vec<_>>();

    let client = Client::Client::new(&host, &token);

    // - List all replication tasks
    let all_tasks = client
        .List_replication_tasks()
        .expect("failed to list replication tasks");

    let mut tasks = all_tasks
        .iter()
        .filter(|(name, _)| tasks.contains(&name.as_str()))
        .collect::<HashMap<_, _>>();

    // - Import all datasets
    for dataset in &datasets {
        Import_dataset(dataset).unwrap();
    }
    info!("Imported all datasets");

    println!("{:?}", tasks);

    // - Send replication requests
    for (_, id) in &tasks {
        client.Send_replication_request(**id).unwrap();
    }
    info!("Sent replication requests");

    //- Check replication task state
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        for (name, id) in tasks.clone() {
            let state = client.Get_replication_task_state(*id).unwrap();
            match state {
                Client::State::Running => {
                    info!("Replication task {} is running", name);
                }
                Client::State::Error => {
                    info!("Replication task {} has an error", name);
                    tasks.remove(name);
                }
                Client::State::Finished => {
                    info!("Replication task {} has succeeded", name);
                    tasks.remove(name);
                }
            }
        }
        if tasks.is_empty() {
            break;
        }
    }

    // - Export all datasets
    for dataset in datasets {
        Export_dataset(dataset).expect("Failed to export dataset");
    }
    info!("Exported all datasets");

    // - Shutdown all disks
    for disk in disks {
        Shutdown_disk(disk).expect("Failed to shutdown disk");
    }
    info!("Shutdown all disks");
}
