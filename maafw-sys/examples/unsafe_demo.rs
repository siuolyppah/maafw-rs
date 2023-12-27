use std::ffi::CString;
use std::ptr::{null, null_mut};
use maafw_sys::{MaaAdbControllerCreateV2, MaaControllerHandle, MaaToolKitGetDeviceAdbConfig, MaaToolKitGetDeviceAdbControllerType, MaaToolKitGetDeviceAdbPath, MaaToolKitGetDeviceAdbSerial, MaaToolKitInit, MaaToolKitPostFindDevice, MaaToolKitWaitForFindDeviceToComplete};

unsafe fn create_adb_controller() -> MaaControllerHandle {
    MaaToolKitPostFindDevice();
    let device_size = MaaToolKitWaitForFindDeviceToComplete();
    if device_size == 0 {
        println!("No device found");
        return MaaControllerHandle::null_mut();
    }

    let idx = 0;    // for demo, we just use the first device
    let agent_path = CString::new("share/MaaAgentBinary").unwrap();
    let controller_handle = MaaAdbControllerCreateV2(
        MaaToolKitGetDeviceAdbPath(idx),
        MaaToolKitGetDeviceAdbSerial(idx),
        MaaToolKitGetDeviceAdbControllerType(idx),
        MaaToolKitGetDeviceAdbConfig(idx),
        agent_path.as_ptr(),
        null(),
        null_mut(),
    );

    controller_handle
}

unsafe fn unsafe_main() {
    MaaToolKitInit();

    let _controller_handle = create_adb_controller();

    unimplemented!()
}

fn main() {
    unsafe {
        unsafe_main();
    }
}