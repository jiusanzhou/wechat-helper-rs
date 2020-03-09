extern crate winapi;

use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE};
use winapi::um::winnt::{DLL_PROCESS_DETACH, DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH};
use winapi::um::libloaderapi::{DisableThreadLibraryCalls};

mod example;

use example::message_box_w;

// define dllmain to handle the init action
#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
  match reason {
    DLL_PROCESS_DETACH => {}
    DLL_PROCESS_ATTACH => unsafe {
        println!("i have attach the main process.");
        message_box_w("Hello WeChat", "I'm injected by Rust.").unwrap();
        DisableThreadLibraryCalls(hinst);
    },
    DLL_THREAD_ATTACH => {}
    DLL_THREAD_DETACH => {}
    _ => {}
  };

  return TRUE;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
