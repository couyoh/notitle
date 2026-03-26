use core::{ffi::c_void, fmt};

// https://uefi.org/specs/UEFI/2.11/Apx_D_Status_Codes.html
pub type EfiStatus = usize;
pub const EFI_SUCCESS: EfiStatus = 0;

type DummyFn = extern "efiapi" fn();

// https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id4
#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

// https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: EfiTextReset,
    pub output_string: EfiTextString,
    pub test_string: DummyFn,
    pub query_mode: DummyFn,
    pub set_mode: DummyFn,
    pub set_attribute: DummyFn,
    pub clear_screen: DummyFn,
    pub set_cursor_position: DummyFn,
    pub enable_cursor: DummyFn,
    pub mode: DummyFn,
}

pub type EfiTextReset = extern "efiapi" fn(
    this: &EfiSimpleTextOutputProtocol,
    extended_verification: bool,
) -> EfiStatus;

pub type EfiTextString =
    extern "efiapi" fn(this: &EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus;

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }
    pub fn output_string(&self, string: *const u16) -> EfiStatus {
        (self.output_string)(self, string)
    }
}

// https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6
#[repr(C)]
pub struct EfiSystemTable<'a> {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: DummyFn,
    pub firmware_revision: DummyFn,
    pub console_in_handle: DummyFn,
    pub con_in: DummyFn,
    pub console_out_handle: DummyFn,
    pub con_out: &'a EfiSimpleTextOutputProtocol,
    pub standard_error_handle: DummyFn,
    pub std_err: DummyFn,
    pub runtime_services: DummyFn,
    pub boot_services: &'a EfiBootServices<'a>,
    pub number_of_table_entries: DummyFn,
    pub configuration_table: DummyFn,
}

// https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface
pub type EfiHandle = *const c_void;
#[repr(C)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
// https://uefi.org/specs/UEFI/2.11/09_Protocols_EFI_Loaded_Image.html#id3
pub const EFI_LOADED_IMAGE_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data1: 0x5B1B31A1,
    data2: 0x9562,
    data3: 0x11d2,
    data4: [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

// https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-openprotocol
pub type EfiOpenProtocol = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    interface: *mut *mut c_void,
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
    attributes: u32,
) -> EfiStatus;

// https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-handleprotocol
pub type EfiHandleProtocol = extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    interface: *mut *mut c_void,
) -> EfiStatus;

// https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-boot-services
#[repr(C)]
pub struct EfiBootServices<'a> {
    pub hdr: DummyFn,
    pub raise_tpl: DummyFn,
    pub restore_tpl: DummyFn,
    pub allocate_pages: DummyFn,
    pub free_pages: DummyFn,
    pub get_memory_map: DummyFn,
    pub allocate_pool: DummyFn,
    pub free_pool: DummyFn,
    pub create_event: DummyFn,
    pub set_timer: DummyFn,
    pub wait_for_event: DummyFn,
    pub signal_event: DummyFn,
    pub close_event: DummyFn,
    pub check_event: DummyFn,
    pub install_protocol_interface: DummyFn,
    pub reinstall_protocol_interface: DummyFn,
    pub uninstall_protocol_interface: DummyFn,
    pub handle_protocol: &'a EfiHandleProtocol,
    pub reserved: DummyFn,
    pub register_protocol_notify: DummyFn,
    pub locate_handle: DummyFn,
    pub locate_device_path: DummyFn,
    pub install_configuration_table: DummyFn,
    pub load_image: DummyFn,
    pub start_image: DummyFn,
    pub exit: DummyFn,
    pub unload_image: DummyFn,
    pub exit_boot_services: DummyFn,
    pub get_next_monotonic_count: DummyFn,
    pub stall: DummyFn,
    pub set_watchdog_timer: DummyFn,
    pub connect_controller: DummyFn,
    pub disconnect_controller: DummyFn,
    pub open_protocol: EfiOpenProtocol,
    pub close_protocol: DummyFn,
    pub open_protocol_information: DummyFn,
    pub protocols_per_handle: DummyFn,
    pub locate_handle_buffer: DummyFn,
    pub locate_protocol: DummyFn,
    pub install_multiple_protocol_interfaces: DummyFn,
    pub uninstall_multiple_protocol_interfaces: DummyFn,
    pub calculate_crc32: DummyFn,
    pub copy_mem: DummyFn,
    pub set_mem: DummyFn,
    pub create_event_ex: DummyFn,
}

impl EfiBootServices<'_> {
    pub fn handle_protocol(
        &self,
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut *mut c_void,
    ) -> EfiStatus {
        (self.handle_protocol)(handle, protocol, interface)
    }
}

// https://uefi.org/specs/UEFI/2.11/09_Protocols_EFI_Loaded_Image.html#id3
#[repr(C)]
pub enum EfiMemoryType {
    ReservedMemoryType = 0,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    AcpiReclaimMemory,
    AcpiMemoryNvs,
    MemoryMappedIo,
    MemoryMappedIoPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
    MaxMemoryType,
}

// https://uefi.org/specs/UEFI/2.11/09_Protocols_EFI_Loaded_Image.html#efi-loaded-image-protocol
#[repr(C)]
pub struct EfiLoadedImageProtocol {
    pub revision: u32,
    pub parent_handle: EfiHandle,
    pub system_table: *mut c_void,

    pub device_handle: EfiHandle,
    pub file_path: *mut c_void,
    pub reserved: *mut c_void,

    pub load_options_size: u32,
    pub load_options: *mut c_void,

    // Location where image was loaded
    pub image_base: *mut c_void,
    pub image_size: u64,
    // pub image_code_type: EfiMemoryType,
    // pub image_data_type: EfiMemoryType,
    // pub unload: DummyFn,
}

pub struct Writer<'a> {
    pub protocol: &'a EfiSimpleTextOutputProtocol,
}

impl fmt::Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.encode_utf16() {
            self.protocol.output_string(&[c, 0] as *const u16);
        }

        Ok(())
    }
}
