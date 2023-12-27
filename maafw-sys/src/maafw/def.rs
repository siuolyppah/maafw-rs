//! Corresponding cpp source file path: ${ROOT}/include/MaaFramework/MaaDef.h

use libc::{c_char, c_void};
use crate::{cenum, opaque};

pub type MaaBool = u8;
pub type MaaSize = u64;

#[repr(C)]
pub struct MaaResourceAPI;
opaque!(MaaResourceHandle, underlying=MaaResourceAPI);

#[repr(C)]
pub struct MaaControllerAPI;
opaque!(MaaControllerHandle, underlying=MaaControllerAPI);

pub type MaaId = i64;
pub type MaaResId = MaaId;

pub type MaaStringView = *const c_char;

opaque!(MaaTransparentArg);
pub type MaaCallbackTransparentArg = MaaTransparentArg;

pub type MaaAPICallback = *extern "C" fn(msg: MaaStringView, details_json: MaaStringView, callback_arg: *mut c_void);
pub type MaaResourceCallback = MaaAPICallback;
pub type MaaControllerCallback = MaaAPICallback;
pub type MaaInstanceCallback = MaaAPICallback;



cenum! {
    enum MaaAdbControllerTypeEnum : i32, type_name=MaaAdbControllerType {
        MaaAdbControllerType_Invalid = 0,

        MaaAdbControllerType_Touch_Adb = 1,
        MaaAdbControllerType_Touch_MiniTouch = 2,
        MaaAdbControllerType_Touch_MaaTouch = 3,
        MaaAdbControllerType_Touch_Mask = 0xFF,

        MaaAdbControllerType_Key_Adb = 1 << 8,
        MaaAdbControllerType_Key_MaaTouch = 2 << 8,
        MaaAdbControllerType_Key_Mask = 0xFF00,

        MaaAdbControllerType_Input_Preset_Adb = MaaAdbControllerType_Touch_Adb | MaaAdbControllerType_Key_Adb,
        MaaAdbControllerType_Input_Preset_Minitouch = MaaAdbControllerType_Touch_MiniTouch | MaaAdbControllerType_Key_Adb,
        MaaAdbControllerType_Input_Preset_Maatouch =
            MaaAdbControllerType_Touch_MaaTouch | MaaAdbControllerType_Key_MaaTouch,

        MaaAdbControllerType_Screencap_FastestWay = 1 << 16,
        MaaAdbControllerType_Screencap_RawByNetcat = 2 << 16,
        MaaAdbControllerType_Screencap_RawWithGzip = 3 << 16,
        MaaAdbControllerType_Screencap_Encode = 4 << 16,
        MaaAdbControllerType_Screencap_EncodeToFile = 5 << 16,
        MaaAdbControllerType_Screencap_MinicapDirect = 6 << 16,
        MaaAdbControllerType_Screencap_MinicapStream = 7 << 16,
        MaaAdbControllerType_Screencap_Mask = 0xFF0000,
    }
}



