use std::env;
use synthetic_brain_hal::{BrainMessage, CognitiveState, FaultInjection, FaultKind, HealthTelemetry, MessageBus, NeuroLinkFrame, Scheduler, VirtualCpu, VirtualGpu, VirtualMemory, VirtualNeuroLink, VirtualPower, VirtualStorage, VirtualThermal, VirtualWatchdog};

const VERSION: &str = "1.0.5";
const RELEASE: &str = "Prototype Alpha 1.0.5";
const DEV_NAME: &str = "ArkansasIo";
const BUILD_ID: &str = "SBA-PA1-0105";
const REGION_COUNT: usize = 12;

#[derive(Debug, Clone, Copy)]
struct RuntimeOptions { ticks: u64, headless: bool }

fn parse_options() -> RuntimeOptions {
    let mut options = RuntimeOptions { ticks: 10, headless: false };
    let args: Vec<String> = env::args().collect();
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--version" | "-V" => {
                println!("Synthetic Brain Architecture {VERSION} ({BUILD_ID})");
                std::process::exit(0);
            }
            "--headless" => options.headless = true,
            "--ticks" => {
                if let Some(value) = args.get(index + 1) { options.ticks = value.parse().unwrap_or(options.ticks); index += 1; }
            }
            "--help" | "-h" => {
                println!("Usage: human-brain [--ticks N] [--headless] [--version]");
                std::process::exit(0);
            }
            _ => {}
        }
        index += 1;
    }
    options
}

#[derive(Debug, Clone, Copy)]
struct Region { id: usize, name: &'static str, enabled: bool }

fn regions() -> [Region; REGION_COUNT] {
    [
        Region { id: 0, name: "brainstem", enabled: true }, Region { id: 1, name: "thalamus", enabled: true },
        Region { id: 2, name: "cerebellum", enabled: true }, Region { id: 3, name: "hippocampus", enabled: true },
        Region { id: 4, name: "amygdala", enabled: true }, Region { id: 5, name: "basal-ganglia", enabled: true },
        Region { id: 6, name: "frontal", enabled: true }, Region { id: 7, name: "parietal", enabled: true },
        Region { id: 8, name: "temporal", enabled: true }, Region { id: 9, name: "occipital", enabled: true },
        Region { id: 10, name: "motor", enabled: true }, Region { id: 11, name: "somatosensory", enabled: true },
    ]
}

fn main() {
    let options = parse_options();
    println!("Synthetic Brain Architecture");
    println!("Developer : {DEV_NAME}");
    println!("Version   : {VERSION}");
    println!("Release   : {RELEASE}");
    println!("Build ID  : {BUILD_ID}");
    println!("Target    : virtual hardware");
    println!("Control   : ticks={} headless={}", options.ticks, options.headless);

    let regions = regions();
    if !options.headless {
        println!("Initializing {} regional cartridges...", regions.len());
        for region in regions { println!("  [{}] {} -> {}", region.id, region.name, if region.enabled { "READY" } else { "DISABLED" }); }
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
    let mut injected_fault = FaultInjection::new(FaultKind::SensorFailure, 0);
    injected_fault.clear();

    let _ = memory.allocate(1024 * 1024);
    let _ = storage.write(4096);
    if !options.headless {
        println!("Neuro-Link backplane: ONLINE ({} lanes)", neuro_link.lane_count);
        println!("Virtual compute: {} CPU cores + {} accelerator units", cpu.cores, gpu.units);
        println!("Virtual memory/storage: ONLINE");
        println!("Virtual thermal/power/watchdog: ONLINE at {:.1} V", power.input_v);
        println!("Cognitive runtime: STARTING");
    }

    for _ in 0..options.ticks {
        let tick = scheduler.step();
        cpu.execute(1000); gpu.submit(1); thermal.step(0.25); watchdog.kick(tick.index); neuro_link.transmit();
        cognitive.perceive(0.5); cognitive.evaluate(0.1, 0.4); cognitive.decide(1, 0.4);
        bus.send(BrainMessage { timestamp: tick.index, source: 0, destination: 6, channel: 1, priority: 1, payload: vec![1, 2, 3] });
        let _ = bus.poll();
        let telemetry = HealthTelemetry::evaluate(tick.index, thermal.temperature_c, power.input_v, watchdog.expired(tick.index), thermal.emergency());
        let frame = NeuroLinkFrame::new(0, 6, 1, 1, tick.index as u32, tick.index, vec![1, 2, 3]);
        let _encoded_frame = frame.encode();
        if !options.headless { println!("tick={} dt={:.3}s temp={:.1}C health={:?} queue={}", tick.index, tick.dt_seconds, telemetry.temperature_c, telemetry.state, bus.len()); }
    }

    println!("Watchdog expired: {}", watchdog.expired(scheduler.index));
    println!("Thermal emergency: {}", thermal.emergency());
    println!("Neuro-Link TX frames: {}", neuro_link.frames_tx);
    println!("Prototype Alpha runtime test complete.");
}
