// 한글 주석 | english comment
// 윈도우 부팅 시에 실행됩니다 | Executed when the window is loaded
#![windows_subsystem = "windows"] // 백그라운드 프로세스로 설정 | Set as background process
use std::fs::*;
use std::env::var;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::Foundation::*;

fn combine_path<'a>(env:&'a str, addpath:&'a str) -> String {
    match var(env) {
        Ok(env) =>
            return format!("{}{}", env, addpath),
        Err(error) =>
            unsafe {
                MessageBoxA(0,
                            b"Shared Notepad Windows Component\0".as_ptr(),
                            format!("Please install {}!\n{}\0", env, error).as_ptr(),
                            MB_OK);
                return format!("{}", error);
            }
    };
}

fn main() {
    let local_file_path = combine_path("USERPROFILE", "\\AppData\\Local\\Packages\\Microsoft.MicrosoftStickyNotes_8wekyb3d8bbwe\\LocalState\\plum.sqlite");
    let onedrive_file_path = combine_path("OneDrive", "\\do_not_delete.sqlite");
    let local_file_metadata = metadata(&local_file_path).unwrap().modified().unwrap();
    let onedrive_file_metadata = metadata(&onedrive_file_path).unwrap().modified().unwrap();

    if local_file_metadata > onedrive_file_metadata {
        copy(&local_file_path, &onedrive_file_path); // 복사 붙여넣기 덮어쓰기 | copy paste overwrite
    } else if local_file_metadata < onedrive_file_metadata {
        copy(&onedrive_file_path, &local_file_path); // 복사 붙여넣기 덮어쓰기 | copy paste overwrite
    }
}