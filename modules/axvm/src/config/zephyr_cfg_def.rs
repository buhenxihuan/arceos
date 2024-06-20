use alloc::vec::Vec;

use page_table_entry::MappingFlags;

use crate::config::GUEST_PHYS_MEMORY_BASE;
use crate::config::GUEST_PHYS_MEMORY_SIZE;
use crate::mm::GuestMemoryRegion;

pub fn zephyr_memory_regions_setup(regions: &mut Vec<GuestMemoryRegion>) {
    let guest_memory_regions = [
        // 0x0000_0000 ~ 0x0800_0000 (0m ~ 128m)
        GuestMemoryRegion {
            // Low RAM1
            gpa: GUEST_PHYS_MEMORY_BASE,
            hpa: 0,
            size: 0x800_0000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
        },
        // 0x7000_0000 ~ 0x8000_0000
        GuestMemoryRegion {
            // RAM
            gpa: 0x7000_0000,
            hpa: 0,
            size: 0x1000_0000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
        },
        GuestMemoryRegion {
            // mmio
            gpa: 0xb0000000,
            hpa: 0xb0000000,
            size: 0x1000_0000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        },
        GuestMemoryRegion {
            gpa: 0xfe00_0000,
            hpa: 0xfe00_0000,
            size: 0xc0_0000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        },
        GuestMemoryRegion {
            // IO APIC
            gpa: 0xfec0_0000,
            hpa: 0xfec0_0000,
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        },
        GuestMemoryRegion {
            // HPET
            gpa: 0xfed0_0000,
            hpa: 0xfed0_0000,
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        },
        GuestMemoryRegion {
            // Local APIC
            gpa: 0xfee0_0000,
            hpa: 0xfee0_0000,
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        },
    ];
    for r in guest_memory_regions {
        regions.push(r);
    }
}
