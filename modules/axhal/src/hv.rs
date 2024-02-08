use crate::mem::{phys_to_virt, virt_to_phys, PAGE_SIZE_4K};
use axalloc::global_allocator;
use hypercraft::{HostPhysAddr, HostVirtAddr, HyperCraftHal};

use crate_interface::{call_interface, def_interface};

#[def_interface]
pub trait VMExitHandler {
    /// Handle hypercall.
    fn vmexit_handler(vcpu: &mut hypercraft::VCpu<HyperCraftHalImpl>) -> hypercraft::HyperResult;
}

#[allow(dead_code)]
#[no_mangle]
fn handle_vmexit(vcpu: &mut hypercraft::VCpu<HyperCraftHalImpl>) -> hypercraft::HyperResult {
    call_interface!(VMExitHandler::vmexit_handler, vcpu)
}

/// An empty struct to implementate of `HyperCraftHal`
pub struct HyperCraftHalImpl;

impl HyperCraftHal for HyperCraftHalImpl {
    fn alloc_pages(num_pages: usize) -> Option<hypercraft::HostVirtAddr> {
        global_allocator()
            .alloc_pages(num_pages, PAGE_SIZE_4K)
            .map(|pa| pa as HostVirtAddr)
            .ok()
    }

    fn dealloc_pages(pa: HostVirtAddr, num_pages: usize) {
        global_allocator().dealloc_pages(pa as usize, num_pages);
    }

    fn phys_to_virt(pa: HostPhysAddr) -> HostVirtAddr {
        phys_to_virt(pa.into()).into()
    }

    fn virt_to_phys(va: HostVirtAddr) -> HostPhysAddr {
        virt_to_phys(va.into()).into()
    }

    fn current_time_nanos() -> u64 {
        crate::time::current_time_nanos()
    }

    /// VM-Exit handler.
    fn vmexit_handler(vcpu: &mut hypercraft::VCpu<Self>) -> hypercraft::HyperResult {
        handle_vmexit(vcpu)
    }
}

/// This is just for test.
/// VM Vcpu execute interface.
///
/// This trait is defined with the [`#[def_interface]`][1] attribute. 
/// It's implemented with [`#[impl_interface]`][2] in `axvm`.
///
/// [1]: crate_interface::def_interface
/// [2]: crate_interface::impl_interface
#[def_interface]
pub trait VMExecuteInterface {
    /// VM Execute.
    fn vm_run_vcpu(vm_id: usize, vcpu_id: usize) -> bool;
}

#[allow(dead_code)]
#[no_mangle]
pub fn vm_run_vcpu(vm_id: usize, vcpu_id: usize) -> bool {
    call_interface!(VMExecuteInterface::vm_run_vcpu, vm_id, vcpu_id)
}