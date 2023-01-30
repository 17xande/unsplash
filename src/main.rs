use dotenv::dotenv;
use std::env;
use unsplash;
use windows::{
    core::*,
    Win32::{System::Com::*, UI::Shell::*},
};

use std::path::PathBuf;

#[tokio::main]
async fn main() {
    println!("starting");

    dotenv().ok();
    let path = env::current_dir().unwrap().join("photo.jpg");

    unsplash::download_photo(&path).await;
    change_wallpaper(path);
}

fn change_wallpaper(path: PathBuf) {
    let path = path.into_os_string().into_string().unwrap();
    let img = &HSTRING::from(path);

    unsafe {
        // TODO: handle errors better.
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        let dw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap();
        let m_count = dw.GetMonitorDevicePathCount().unwrap();
        println!("detected {} monitors.", m_count);

        for i in 0..m_count {
            let mdp = dw.GetMonitorDevicePathAt(i).unwrap();
            println!("monitor device path: {}", mdp.to_string().unwrap());
            let monitor_id = PCWSTR(mdp.as_ptr());
            dw.SetWallpaper(monitor_id, img).unwrap();
        }

        CoUninitialize();
    }
}
