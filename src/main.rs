use btleplug::api::{Central, Manager as ApiManager, Peripheral as ApiPeripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Constructing new manager...");
    let manager = Manager::new().await.unwrap();

    println!("Constructing adapters...");
    let adapters = manager.adapters().await?;

    println!("Constructing central adapter...");
    let central = adapters.into_iter().next().unwrap();

    println!("Beginning scan...");
    central.start_scan(ScanFilter::default()).await?;

    println!("Sleeping for 2 seconds.");
    time::sleep(Duration::from_secs(3)).await;

    println!("Waking. Searching found devices...");
    let pm5 = find_pm5(&central).await;

    if pm5.is_some() {
        println!("Found pm5!");
        return Ok(());
    } else {
        println!("Did not find pm5...");
    }

    Ok(())
}

async fn find_pm5(central: &Adapter) -> Option<Peripheral> {
    dbg!(central.peripherals().await.unwrap().len());
    for p in central.peripherals().await.unwrap() {
        let props = p.properties().await;
        if props.unwrap().unwrap().local_name.unwrap().contains("PM5") {
            return Some(p);
        }
    }

    None
}
