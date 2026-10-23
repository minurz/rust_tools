
use std::{thread, time::Duration};
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::System::SystemServices::SS_CENTER;
use windows_sys::Win32::Graphics::Gdi::UpdateWindow;

pub fn show_tooltip(message: &'static str) {
    thread::spawn(move || unsafe {
        let class = widestring("STATIC");
        let text = widestring(message);

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            class.as_ptr(),
            text.as_ptr(),
            WS_POPUP | WS_BORDER | SS_CENTER,
            0, 0, 75, 15,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null(),
        );

        if hwnd.is_null() {
            return;
        }

        // Position near the top-right of a typical 1920x1080 display.
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            15, 15, 60, 20,
            SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );

        ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        UpdateWindow(hwnd);

        let start = std::time::Instant::now();
        let mut msg = std::mem::zeroed();

        while start.elapsed() < Duration::from_millis(1000) {
            while PeekMessageW(
                &mut msg,
                hwnd,
                0,
                0,
                PM_REMOVE,
            ) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        DestroyWindow(hwnd);
    });
}

fn widestring(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
