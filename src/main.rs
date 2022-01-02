#![feature(once_cell)]

use btleplug::api::{
    Central, Manager as ApiManager, Peripheral as ApiPeripheral, ScanFilter, Service,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

pub mod enums;
pub mod helpers;
pub mod ids;
pub mod responses;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // PM5 service UUIDS from https://www.c2forum.com/viewtopic.php?f=15&t=81699&p=295721&hilit=uuid#p284373
    println!("Constructing new manager...");
    let manager = Manager::new().await.unwrap();

    println!("Constructing adapters...");
    let adapters = manager.adapters().await?;

    println!("Constructing central adapter...");
    let central = adapters.into_iter().next().unwrap();

    println!("Beginning scan...");
    central.start_scan(ScanFilter::default()).await?;

    //TODO poll stream rather than sleep
    println!("Sleeping for 2 seconds.");
    time::sleep(Duration::from_secs(3)).await;

    println!("Waking. Searching found devices...");

    if let Some(pm5) = find_pm5(&central).await {
        println!("Found pm5!");
        pm5.connect().await?;
        println!("Connected.");
        pm5.discover_services().await?;
        let services = pm5.services();

        // Check that we found 3 of the main PM5 services
        assert!(services
            .iter()
            .any(|s| s.uuid == *ids::services::DEVICEINFO));
        assert!(services.iter().any(|s| s.uuid == *ids::services::CONTROL));
        assert!(services.iter().any(|s| s.uuid == *ids::services::ROWING));
        println!("All required services found.");

        // sub_rowing_status(&pm5, &services).await?;

        sub_force_curve(&pm5, &services).await?;

        pm5.disconnect().await?;
    } else {
        println!("Did not find pm5...");
    }

    Ok(())
}

async fn sub_force_curve(
    pm5: &Peripheral,
    services: &std::collections::BTreeSet<Service>,
) -> Result<(), Box<dyn Error>> {
    let rowing_service = services
        .iter()
        .find(|s| s.uuid == *ids::services::ROWING)
        .unwrap();

    let forcecurve_char = rowing_service
        .characteristics
        .iter()
        .find(|c| c.uuid == *ids::chars::FORCECURVE)
        .unwrap();

    println!("Subscribing to force curve characteristic...");
    pm5.subscribe(forcecurve_char).await?;
    println!("Subscription done.");

    // let mut full_data = Vec::new();
    loop {
        // TODO document times in dataset
        let response = pm5.read(forcecurve_char).await?;

        {
            assert!(response.len() >= 2);
            // little endian response?
            let num_words = response[0] & 0x0F;
            let num_characteristics = response[0] >> 2;
            let sequence_number = response[1];

        }
    }
}

#[allow(dead_code)]
async fn sub_rowing_status(
    pm5: &Peripheral,
    services: &std::collections::BTreeSet<Service>,
) -> Result<(), Box<dyn Error>> {
    let rowing_service = services
        .iter()
        .find(|s| s.uuid == *ids::services::ROWING)
        .unwrap();

    let rowing_status_char = rowing_service
        .characteristics
        .iter()
        .find(|c| c.uuid == *ids::chars::ROWING_GENERAL)
        .unwrap();

    println!("Subscribing to rowing characteristic...");
    pm5.subscribe(rowing_status_char).await?;
    println!("Subscription done.");

    for _ in 0..60 {
        let response = pm5.read(rowing_status_char).await?;

        let _parsed = responses::RowingStatusResponse::from_bytes(&response);

        time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn find_pm5(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        let props = p.properties().await;
        let name = props.unwrap().unwrap().local_name;
        if let Some(s) = name.as_ref() {
            println!("Found device with name {:?}", s);
            if s.contains("PM5") {
                return Some(p);
            }
        }
    }

    None
}
