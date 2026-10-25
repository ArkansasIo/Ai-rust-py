use synthetic_brain_hal::{CognitiveState, MessageBus, BrainMessage, Scheduler, VirtualCpu, VirtualGpu, VirtualMemory, VirtualStorage, VirtualThermal, VirtualPower, VirtualWatchdog, VirtualNeuroLink};

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

    let mut cpu = VirtualCpu::new(12);
    let mut gpu = VirtualGpu::new(16);
    let mut memory = VirtualMemory::new(16 * 1024 * 1024 * 1024usize);
    let mut storage = VirtualStorage::new(128 * 1024 * 1024 * 1024u64);
    let mut thermal = VirtualThermal::new();
    let power = VirtualPower::default();
    let mut watchdog = VirtualWatchdog::new(20);
    let mut neuro_link = VirtualNeuroLink::new(12);
    let mut bus = MessageBus::default();
    let mut scheduler = Scheduler::new(10);
    let mut cognitive = CognitiveState::new();

    let _ = memory.allocate(1024 * 1024);
    let _ = storage.write(4096);
    println!("Neuro-Link backplane: ONLINE ({} lanes)", neuro_link.lane_count);
    println!("Virtual compute: {} CPU cores + {} accelerator units", cpu.cores, gpu.units);
    println!("Virtual memory/storage: ONLINE");
    println!("Virtual thermal/power/watchdog: ONLINE at {:.1} V", power.input_v);
    println!("Cognitive runtime: STARTING");

    for _ in 0..10 {
        let tick = scheduler.step();
        cpu.execute(1000);
        gpu.submit(1);
        thermal.step(0.25);
        watchdog.kick(tick.index);
        neuro_link.transmit();
        cognitive.perceive(0.5);
        cognitive.evaluate(0.1, 0.4);
        cognitive.decide(1, 0.4);
        bus.send(BrainMessage { timestamp: tick.index, source: 0, destination: 6, channel: 1, priority: 1, payload: vec![1, 2, 3] });
        let _ = bus.poll();
        println!("tick={} dt={:.3}s temp={:.1}C queue={}", tick.index, tick.dt_seconds, thermal.temperature_c, bus.len());
    }

    println!("Watchdog expired: {}", watchdog.expired(scheduler.index));
    println!("Thermal emergency: {}", thermal.emergency());
    println!("Prototype Alpha runtime test complete.");
}
