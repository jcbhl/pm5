use btleplug::api::{Central, Manager as ApiManager, Peripheral as ApiPeripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

pub mod enums;
pub mod helpers;
pub mod responses;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // PM5 service UUIDS from https://www.c2forum.com/viewtopic.php?f=15&t=81699&p=295721&hilit=uuid#p284373
    // FIXME constexprs
    let deviceinfo_uuid = Uuid::parse_str("CE060010-43E5-11E4-916C-0800200C9A66").unwrap();
    let controlservice_uuid = Uuid::parse_str("CE060020-43E5-11E4-916C-0800200C9A66").unwrap();
    let rowingservice_uuid = Uuid::parse_str("CE060030-43E5-11E4-916C-0800200C9A66").unwrap();

    let rowingstatus_characteristic_uuid =
        Uuid::parse_str("ce060031-43e5-11e4-916c-0800200c9a66").unwrap();

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
        assert!(services.iter().any(|s| s.uuid == deviceinfo_uuid));
        assert!(services.iter().any(|s| s.uuid == controlservice_uuid));
        assert!(services.iter().any(|s| s.uuid == rowingservice_uuid));
        println!("All required services found.");

        let rowing_service = services
            .iter()
            .find(|s| s.uuid == rowingservice_uuid)
            .unwrap();

        let rowing_status_char = rowing_service
            .characteristics
            .iter()
            .find(|c| c.uuid == rowingstatus_characteristic_uuid)
            .unwrap();

        println!("Subscribing to rowing characteristic...");
        pm5.subscribe(rowing_status_char).await?;
        println!("Subscription done.");

        for _ in 0..60 {
            let response = pm5.read(rowing_status_char).await?;

            time::sleep(Duration::from_secs(1)).await;
        }

        pm5.disconnect().await?;
    } else {
        println!("Did not find pm5...");
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
