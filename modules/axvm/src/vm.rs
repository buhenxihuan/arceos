use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use super::arch::VCpu;
use crate::Result;
use axhal::hv::HyperCraftHalImpl;

use super::device::{self, X64VcpuDevices, X64VmDevices};
pub use hypercraft::{HyperCraftHal, HyperError, PerCpuDevices, PerVmDevices, VmCpus, VM};

pub const CONFIG_VM_NUM_MAX: usize = 8;

static VM_LIST: Mutex<
    Vec<
        Arc<
            VM<
                HyperCraftHalImpl,
                X64VcpuDevices<HyperCraftHalImpl>,
                X64VmDevices<HyperCraftHalImpl>,
            >,
        >,
    >,
> = Mutex::new(Vec::new());

#[inline]
pub fn vm_list_walker<F>(mut f: F)
where
    F: FnMut(
        &Arc<
            VM<
                HyperCraftHalImpl,
                X64VcpuDevices<HyperCraftHalImpl>,
                X64VmDevices<HyperCraftHalImpl>,
            >,
        >,
    ),
{
    let vm_list = VM_LIST.lock();
    for vm in vm_list.iter() {
        f(vm);
    }
}

pub fn push_vm(
    id: usize,
    vcpus: VmCpus<HyperCraftHalImpl, X64VcpuDevices<HyperCraftHalImpl>>,
) -> Result<
    Arc<VM<HyperCraftHalImpl, X64VcpuDevices<HyperCraftHalImpl>, X64VmDevices<HyperCraftHalImpl>>>,
> {
    let mut vm_list = VM_LIST.lock();

    let mut vm = VM::<
        HyperCraftHalImpl,
        X64VcpuDevices<HyperCraftHalImpl>,
        X64VmDevices<HyperCraftHalImpl>,
    >::new(id, vcpus);

    let this = Arc::new(vm);

    if id >= CONFIG_VM_NUM_MAX || vm_list.iter().any(|x| x.id() == id) {
        error!("push_vm: vm {} already exists", id);
        Err(HyperError::OutOfRange)
    } else {
        let vm = this;
        vm_list.push(vm.clone());
        Ok(vm)
    }
}

pub fn remove_vm(
    id: usize,
) -> Arc<VM<HyperCraftHalImpl, X64VcpuDevices<HyperCraftHalImpl>, X64VmDevices<HyperCraftHalImpl>>>
{
    let mut vm_list = VM_LIST.lock();
    match vm_list.iter().position(|x| x.id() == id) {
        None => {
            panic!("VM[{}] not exist in VM LIST", id);
        }
        Some(idx) => vm_list.remove(idx),
    }
}

pub fn vm_by_id(
    id: usize,
) -> Option<
    Arc<VM<HyperCraftHalImpl, X64VcpuDevices<HyperCraftHalImpl>, X64VmDevices<HyperCraftHalImpl>>>,
> {
    let vm_list = VM_LIST.lock();
    vm_list.iter().find(|&x| x.id() == id).cloned()
}

struct VMExecuteInterfaceImpl;

#[crate_interface::impl_interface]
impl axhal::hv::VMExecuteInterface for VMExecuteInterfaceImpl {
    fn vm_run_vcpu(vm_id: usize, vcpu_id: usize) -> bool {
        let vm = vm_by_id(vm_id).expect("VM not exist");

        let _ = vm.run_vcpu(vcpu_id);

        true
    }
}
