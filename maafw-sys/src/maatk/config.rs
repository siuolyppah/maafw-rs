//! Corresponding cpp source file path: ${ROOT}/include/MaaToolKit/Config/MaaToolKitConfig.h


use crate::MaaBool;

extern "C" {
    pub fn MaaToolKitInit() -> MaaBool;
}