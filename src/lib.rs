#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;

// Custom implementation since
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apk_package {
    pub hash_node: apk_hash_node,
    pub __bindgen_anon_1: apk_package__bindgen_ty_1,
    pub name: *mut apk_name,
    pub ipkg: *mut apk_installed_package,
    pub version: *mut apk_blob_t,
    pub arch: *mut apk_blob_t,
    pub license: *mut apk_blob_t,
    pub origin: *mut apk_blob_t,
    pub maintainer: *mut apk_blob_t,
    pub url: *mut ::std::os::raw::c_char,
    pub description: *mut ::std::os::raw::c_char,
    pub commit: *mut ::std::os::raw::c_char,
    pub filename: *mut ::std::os::raw::c_char,
    pub depends: *mut apk_dependency_array,
    pub install_if: *mut apk_dependency_array,
    pub provides: *mut apk_dependency_array,
    pub installed_size: size_t,
    pub size: size_t,
    pub build_time: time_t,
    pub provider_priority: ::std::os::raw::c_ushort,
    pub repos: uint32_t,
    pub marked: uint8_t,
    pub uninstallable: uint8_t,
    pub cached_non_repository: uint8_t,
    pub csum: apk_checksum,
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
