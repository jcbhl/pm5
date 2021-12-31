use btleplug::api::{Central, Manager as ApiManager, Peripheral as ApiPeripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

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
        println!("Found {:?} services", services.len());
        for service in services.iter() {
            println!("UUID found: {:?}", service.uuid);
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
