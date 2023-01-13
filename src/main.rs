use windows::{
    core::*,
    Win32::{System::Com::*, UI::Shell::*},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting");

    change_wallpaper("C:\\Users\\17xan\\dev\\unsplash\\wallpaper.jpg");

    Ok(())
}

fn change_wallpaper(path: &str) {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        let dw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap();

        let mdp = dw.GetMonitorDevicePathAt(0).unwrap();
        println!("monitor device path: {}", mdp.to_string().unwrap());
        let monitor_id = PCWSTR(mdp.as_ptr());
        let img = &HSTRING::from(path);
        println!("desktop wallpaper path: {}", img.to_string());
        dw.SetWallpaper(monitor_id, img).unwrap();

        CoUninitialize();
    }
}
