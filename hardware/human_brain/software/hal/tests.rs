//! HAL integration-style unit tests for deterministic virtual hardware.

use super::*;

#[test]
fn memory_allocation_and_free_are_bounded() {
    let mut memory = VirtualMemory::new(1024);
    assert!(memory.allocate(512));
    assert_eq!(memory.used_bytes, 512);
    assert!(!memory.allocate(513));
    memory.free(256);
    assert_eq!(memory.used_bytes, 256);
}

#[test]
fn thermal_model_reports_emergency() {
    let mut thermal = VirtualThermal::new();
    thermal.temperature_c = 84.9;
    thermal.step(1.0);
    assert!(thermal.emergency());
}

#[test]
fn watchdog_expires_after_timeout() {
    let mut watchdog = VirtualWatchdog::new(3);
    watchdog.kick(10);
    assert!(!watchdog.expired(13));
    assert!(watchdog.expired(14));
}

#[test]
fn neuro_link_tracks_frames() {
    let mut link = VirtualNeuroLink::new(12);
    link.transmit();
    link.receive();
    assert_eq!(link.lane_count, 12);
    assert_eq!(link.frames_tx, 1);
    assert_eq!(link.frames_rx, 1);
}

#[test]
fn management_buses_record_activity() {
    let mut gpio = VirtualGpio::default();
    gpio.set(3, true);
    assert!(gpio.pins[3]);

    let mut i2c = VirtualI2c::default();
    i2c.transfer();
    assert_eq!(i2c.transactions, 1);

    let mut spi = VirtualSpi::default();
    spi.transfer();
    assert_eq!(spi.transactions, 1);

    let mut uart = VirtualUart::default();
    uart.transmit(2);
    uart.receive(3);
    assert_eq!(uart.tx_bytes, 2);
    assert_eq!(uart.rx_bytes, 3);
}

#[test]
fn virtual_compute_and_storage_progress() {
    let mut cpu = VirtualCpu::new(12);
    cpu.execute(100);
    assert_eq!(cpu.instructions, 100);

    let mut gpu = VirtualGpu::new(16);
    gpu.submit(4);
    assert_eq!(gpu.jobs_completed, 4);

    let mut storage = VirtualStorage::new(1024);
    assert!(storage.write(512));
    assert!(!storage.write(513));
}
