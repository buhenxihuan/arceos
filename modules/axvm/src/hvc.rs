use core::iter::Map;

use page_table_entry::MappingFlags;

use crate::{mm::MapRegion, HyperCraftHal, HypercallArgs, Result, Error as HyperError, VCpu};
// use axhal::hv::HyperCraftHalImpl;

pub const HVC_SHADOW_PROCESS_READY: usize           = 0x53686477; // "Shdw"
pub const HVC_SHADOW_PROCESS_READY_MAGIC_0: u32     = 0x70726373; // "prcs"
pub const HVC_SHADOW_PROCESS_READY_MAGIC_1: u32     = 0x52647921; // "Rdy!"
pub const HVC_EPT_MAPPING_REQUEST: usize            = 0x454d6170; // "EMap"

pub fn handle_hvc<H: HyperCraftHal>(vcpu: &mut VCpu<H>, id: usize, args: HypercallArgs) -> Result<u32> {
    info!(
        "hypercall_handler vcpu: {}, id: {:#x?}, args: {:#x?}",
        vcpu.vcpu_id(),
        id,
        args,
    );

    match id {
        HVC_SHADOW_PROCESS_READY => {
            if args[0] != HVC_SHADOW_PROCESS_READY_MAGIC_0 || args[1] != HVC_SHADOW_PROCESS_READY_MAGIC_1 {
                warn!("Invalid magic number for hypercall shadow_process_ready. vcpu: {:#x?}", vcpu);
                return Err(HyperError::InvalidParam);
            }
            axtask::notify_all_process();
        },
        HVC_EPT_MAPPING_REQUEST => {
            let hpa = (args[0] as usize) << 32 | args[1] as usize;
            let gpa = (args[2] as usize) << 32 | args[3] as usize;
            let size = args[4] as usize;

            super::linux::get_linux_gpm().map_region(MapRegion::new_offset(gpa, hpa, size, MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE)).unwrap();
            info!("EPT mapping request: hpa: {:#x}, gpa: {:#x}, size: {:#x}", hpa, gpa, size);
        },
        _ => {
            warn!("Unhandled hypercall {}. vcpu: {:#x?}", id, vcpu);
        }
    }
    Ok(0)
    // Err(HyperError::NotSupported)
}
