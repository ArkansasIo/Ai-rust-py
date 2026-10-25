use std::{thread, time::{Duration, Instant}};

const VERSION: &str = "1.0.5";
const RELEASE: &str = "Prototype Alpha 1.0.5";
const DEV_NAME: &str = "ArkansasIo";
const BUILD_ID: &str = "SBA-PA1-0105";
const REGION_COUNT: usize = 12;

#[derive(Debug, Clone, Copy)]
struct Region { id: usize, name: &'static str, enabled: bool }

fn regions() -> [Region; REGION_COUNT] {
    [
        Region { id: 0, name: "brainstem", enabled: true },
        Region { id: 1, name: "thalamus", enabled: true },
        Region { id: 2, name: "cerebellum", enabled: true },
        Region { id: 3, name: "hippocampus", enabled: true },
        Region { id: 4, name: "amygdala", enabled: true },
        Region { id: 5, name: "basal-ganglia", enabled: true },
        Region { id: 6, name: "frontal", enabled: true },
        Region { id: 7, name: "parietal", enabled: true },
        Region { id: 8, name: "temporal", enabled: true },
        Region { id: 9, name: "occipital", enabled: true },
        Region { id: 10, name: "motor", enabled: true },
        Region { id: 11, name: "somatosensory", enabled: true },
    ]
}

fn main() {
    println!("Synthetic Brain Architecture");
    println!("Developer : {DEV_NAME}");
    println!("Version   : {VERSION}");
    println!("Release   : {RELEASE}");
    println!("Build ID  : {BUILD_ID}");
    println!("Target    : virtual hardware");

    let regions = regions();
    println!("Initializing {} regional cartridges...", regions.len());
    for region in regions {
        println!("  [{}] {} -> {}", region.id, region.name, if region.enabled { "READY" } else { "DISABLED" });
    }

    println!("Neuro-Link backplane: ONLINE");
    println!("Virtual sensors: ONLINE");
    println!("Virtual thermal/power/watchdog: ONLINE");
    println!("Cognitive runtime: STARTING");

    let start = Instant::now();
    for tick in 0..10u64 {
        println!("tick={tick} elapsed_ms={}", start.elapsed().as_millis());
        thread::sleep(Duration::from_millis(100));
    }

    println!("Prototype Alpha runtime test complete.");
}
