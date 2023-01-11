use windows::{
    core::*,
    Win32::{System::Com::*, UI::Shell::*},
};

fn main() {
    println!("starting");
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        let dw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap();

        let mdp = dw.GetMonitorDevicePathAt(0).unwrap();
        println!("monitor device path: {}", mdp.to_string().unwrap());
        let monitor_id = PCWSTR(mdp.as_ptr());
        let img = &HSTRING::from("C:\\Users\\17xan\\dev\\unsplash\\wallpaper.jpg");
        println!("desktop wallpaper path: {}", img.to_string());
        dw.SetWallpaper(monitor_id, img).unwrap();

        CoUninitialize();
    }
}
