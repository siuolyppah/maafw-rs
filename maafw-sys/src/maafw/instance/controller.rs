//! Corresponding cpp source file path: ${ROOT}/include/MaaFramework/Instance/MaaController.h

use crate::{MaaAdbControllerType, MaaCallbackTransparentArg, MaaControllerCallback, MaaControllerHandle, MaaStringView};

extern "C" {
    #[deprecated(note = "Use MaaAdbControllerCreateV2 instead")]
    pub fn MaaAdbControllerCreate(
        adb_path: MaaStringView,
        address: MaaStringView,
        config: MaaStringView,
        callback: MaaControllerCallback,
        callback_arg: MaaCallbackTransparentArg,
    ) -> MaaControllerHandle;

    pub fn MaaAdbControllerCreateV2(
        adb_path: MaaStringView,
        address: MaaStringView,
        type_: MaaAdbControllerType,
        config: MaaStringView,
        agent_path: MaaStringView,
        callback: MaaControllerCallback,
        callback_arg: MaaCallbackTransparentArg,
    ) -> MaaControllerHandle;
}