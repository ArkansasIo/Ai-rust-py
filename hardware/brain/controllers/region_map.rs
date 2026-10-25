//! Maps logical brain regions to physical compute controllers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region { Brainstem, Thalamus, Cerebellum, Hippocampus, Amygdala, BasalGanglia, Frontal, Parietal, Temporal, Occipital, Motor, Somatosensory }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControllerSlot { pub bus: u8, pub address: u8, pub region: Region }

pub const DEFAULT_REGION_MAP: &[ControllerSlot] = &[
    ControllerSlot { bus: 0, address: 0x20, region: Region::Brainstem },
    ControllerSlot { bus: 0, address: 0x21, region: Region::Thalamus },
    ControllerSlot { bus: 0, address: 0x22, region: Region::Cerebellum },
    ControllerSlot { bus: 0, address: 0x23, region: Region::Hippocampus },
    ControllerSlot { bus: 0, address: 0x24, region: Region::Amygdala },
    ControllerSlot { bus: 0, address: 0x25, region: Region::BasalGanglia },
    ControllerSlot { bus: 1, address: 0x20, region: Region::Frontal },
    ControllerSlot { bus: 1, address: 0x21, region: Region::Parietal },
    ControllerSlot { bus: 1, address: 0x22, region: Region::Temporal },
    ControllerSlot { bus: 1, address: 0x23, region: Region::Occipital },
    ControllerSlot { bus: 1, address: 0x24, region: Region::Motor },
    ControllerSlot { bus: 1, address: 0x25, region: Region::Somatosensory },
];
