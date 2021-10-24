#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_AI")]
pub mod AI;
#[cfg(feature = "Win32_Data")]
pub mod Data;
#[cfg(feature = "Win32_Devices")]
pub mod Devices;
#[cfg(feature = "Win32_Foundation")]
pub mod Foundation;
#[cfg(feature = "Win32_Gaming")]
pub mod Gaming;
#[cfg(feature = "Win32_Globalization")]
pub mod Globalization;
#[cfg(feature = "Win32_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_Interop")]
pub mod Interop;
#[cfg(feature = "Win32_Management")]
pub mod Management;
#[cfg(feature = "Win32_Media")]
pub mod Media;
#[cfg(feature = "Win32_NetworkManagement")]
pub mod NetworkManagement;
#[cfg(feature = "Win32_Networking")]
pub mod Networking;
#[cfg(feature = "Win32_Security")]
pub mod Security;
#[cfg(feature = "Win32_Storage")]
pub mod Storage;
#[cfg(feature = "Win32_System")]
pub mod System;
#[cfg(feature = "Win32_UI")]
pub mod UI;
#[cfg(feature = "Win32_Web")]
pub mod Web;
