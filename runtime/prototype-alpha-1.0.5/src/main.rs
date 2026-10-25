use std::env;
use synthetic_brain_hal::{BrainMessage, CognitiveState, FaultInjection, FaultKind, HealthTelemetry, MessageBus, NeuroLinkFrame, RegionalManager, Scheduler, VirtualCpu, VirtualGpu, VirtualMemory, VirtualNeuroLink, VirtualPower, VirtualStorage, VirtualThermal, VirtualWatchdog};

const VERSION: &str = "1.0.5";
const RELEASE: &str = "Prototype Alpha 1.0.5";
const DEV_NAME: &str = "ArkansasIo";
const BUILD_ID: &str = "SBA-PA1-0105";

#[derive(Debug, Clone, Copy)]
struct RuntimeOptions { ticks: u64, headless: bool }

fn parse_options() -> RuntimeOptions {
    let mut options = RuntimeOptions { ticks: 10, headless: false };
    let args: Vec<String> = env::args().collect();
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--version" | "-V" => { println!("Synthetic Brain Architecture {VERSION} ({BUILD_ID})"); std::process::exit(0); }
            "--headless" => options.headless = true,
            "--ticks" => { if let Some(value) = args.get(index + 1) { options.ticks = value.parse().unwrap_or(options.ticks); index += 1; } }
            "--help" | "-h" => { println!("Usage: human-brain [--ticks N] [--headless] [--version]"); std::process::exit(0); }
            _ => {}
        }
        index += 1;
    }
    options
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

    let mut regional_manager = RegionalManager::default();
    regional_manager.start_all();
    if !options.headless {
        println!("Regional cartridges: {}/12 READY", regional_manager.ready_count());
        for cartridge in &regional_manager.cartridges { println!("  [{}] {} -> {:?}", cartridge.id, cartridge.name, cartridge.status); }
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
        if !options.headless { println!("tick={} dt={:.3}s temp={:.1}C health={:?} regions={}/12", tick.index, tick.dt_seconds, telemetry.temperature_c, telemetry.state, regional_manager.ready_count()); }
    }

    println!("Watchdog expired: {}", watchdog.expired(scheduler.index));
    println!("Thermal emergency: {}", thermal.emergency());
    println!("Neuro-Link TX frames: {}", neuro_link.frames_tx);
    println!("Regional cartridges ready: {}/12", regional_manager.ready_count());
    println!("Prototype Alpha runtime test complete.");
}
