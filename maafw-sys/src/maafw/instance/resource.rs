//! Corresponding cpp source file path: ${ROOT}/include/MaaFramework/Instance/MaaResource.h

use std::os::raw::c_void;
use crate::{MaaCallbackTransparentArg, MaaResId, MaaResourceCallback, MaaResourceHandle, MaaStringView};

extern "C" {
    pub fn MaaResourceCreate(callback: MaaResourceCallback, callback_arg: MaaCallbackTransparentArg) -> MaaResourceHandle;
    pub fn MaaResourceDestroy(res: MaaResourceHandle) -> c_void;

    pub fn MaaResourcePostPath(res: MaaResourceHandle, path: MaaStringView) -> MaaResId;
}


