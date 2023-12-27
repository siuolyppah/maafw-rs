//! Corresponding cpp source file path: ${ROOT}/include/MaaToolKit/Device/MaaToolKitDevice.h


use crate::{MaaAdbControllerType, MaaBool, MaaSize, MaaStringView};

extern "C" {
    #[deprecated(note = "Use MaaToolKitPostFindDevice instead")]
    pub fn MaaToolKitFindDevice() -> MaaSize;
    #[deprecated(note = "Use MaaToolKitPostFindDeviceWithAdb instead")]
    pub fn MaaToolKitFindDeviceWithAdb(adb_path: MaaStringView) -> MaaSize;

    pub fn MaaToolKitPostFindDevice() -> MaaBool;
    pub fn MaaToolKitPostFindDeviceWithAdb(adb_path: MaaStringView) -> MaaBool;
    pub fn MaaToolKitIsFindDeviceCompleted() -> MaaBool;
    pub fn MaaToolKitWaitForFindDeviceToComplete() -> MaaSize;

    pub fn MaaToolKitGetDeviceCount() -> MaaSize;
    pub fn MaaToolKitGetDeviceName(index: MaaSize) -> MaaStringView;
    pub fn MaaToolKitGetDeviceAdbPath(index: MaaSize) -> MaaStringView;
    pub fn MaaToolKitGetDeviceAdbSerial(index: MaaSize) -> MaaStringView;
    pub fn MaaToolKitGetDeviceAdbControllerType(index: MaaSize) -> MaaAdbControllerType;
    pub fn MaaToolKitGetDeviceAdbConfig(index: MaaSize) -> MaaStringView;
}