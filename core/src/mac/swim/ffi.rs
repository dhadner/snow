use std::os::raw::{c_uint};
use crate::mac::swim::drive::{DriveType, FloppyDrive};
use snow_floppy::{FloppyImage};

#[repr(C)]
pub struct FFIFloppyDrive {
    drive: FloppyDrive,
}

/// Although this is identical to the Rust `DriveType` enum,
/// we define it separately for FFI compatibility to avoid
/// adding '#[repr(C)]' to the Rust enum.
#[repr(C)]
pub enum FFIDriveType {
    None,
    GCR400K,
    GCR800K,
    SuperDrive,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_new(
    idx: c_uint,
    drive_type: FFIDriveType,
    base_frequency: c_uint,
) -> *mut FFIFloppyDrive {
    let rust_type = match drive_type {
        FFIDriveType::None => DriveType::None,
        FFIDriveType::GCR400K => DriveType::GCR400K,
        FFIDriveType::GCR800K => DriveType::GCR800K,
        FFIDriveType::SuperDrive => DriveType::SuperDrive,
    };
    let drive = FloppyDrive::new(idx as usize, rust_type, base_frequency as _);
    Box::into_raw(Box::new(FFIFloppyDrive { drive }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_free(ptr: *mut FFIFloppyDrive) {
    if !ptr.is_null() {
        let _ = unsafe{ Box::from_raw(ptr); };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_is_present(ptr: *const FFIFloppyDrive) -> bool {
    let drv = unsafe { &*ptr };
    drv.drive.is_present()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_disk_insert(
    ptr: *mut FFIFloppyDrive,
    image: *const FloppyImage,
) -> bool {
    let drv = unsafe { &mut *ptr };
    let img = unsafe { &*image };
    drv.drive.disk_insert(img.clone()).is_ok()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_eject(ptr: *mut FFIFloppyDrive) {
    let drv = unsafe { &mut *ptr };
    drv.drive.eject();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_get_track_rpm(ptr: *const FFIFloppyDrive) -> c_uint {
    let drv = unsafe { &*ptr };
    drv.drive.get_track_rpm() as c_uint
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_get_ticks_per_bit(ptr: *const FFIFloppyDrive) -> c_uint {
    let drv = unsafe { &*ptr };
    drv.drive.get_ticks_per_bit() as c_uint
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_get_tacho(ptr: *const FFIFloppyDrive) -> bool {
    let drv = unsafe { &*ptr };
    drv.drive.get_tacho()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn floppy_drive_get_type(ptr: *const FFIFloppyDrive) -> FFIDriveType {
    let drv = unsafe { &*ptr };
    match drv.drive.drive_type {
        DriveType::None => FFIDriveType::None,
        DriveType::GCR400K => FFIDriveType::GCR400K,
        DriveType::GCR800K => FFIDriveType::GCR800K,
        DriveType::SuperDrive => FFIDriveType::SuperDrive,
    }
}