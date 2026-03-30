use njord::connectivity::check_connectivity;
use notify_rust::Notification;
use std::sync::{Arc, Mutex};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::menu::{ContextMenu, MenuItemAttributes};
use tao::system_tray::{Icon, SystemTrayBuilder};
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

const MONITORING_INTERVAL_SECS: u64 = 30;

fn main() {
    let event_loop = EventLoop::new();

    let mut tray_menu = ContextMenu::new();
    tray_menu.add_item(MenuItemAttributes::new("Quit").with_id(tao::menu::MenuId(1)));

    let icon = Icon::from_rgba(vec![255; 32 * 32 * 4], 32, 32).unwrap(); // Placeholder icon

    let _system_tray = SystemTrayBuilder::new(icon, Some(tray_menu))
        .build(&event_loop)
        .unwrap();

    let is_connected = Arc::new(Mutex::new(true));

    let rt = Runtime::new().unwrap();
    let is_connected_clone = Arc::clone(&is_connected);

    rt.spawn(async move {
        loop {
            let connected = check_connectivity().await;
            {
                let mut conn = is_connected_clone.lock().expect("Failed to acquire mutex lock");
                if *conn != connected {
                    *conn = connected;
                    let summary = if connected {
                        "Network connected"
                    } else {
                        "Network disconnected"
                    };
                    Notification::new()
                        .summary(summary)
                        .body("Internet connectivity status changed")
                        .show()
                        .unwrap();
                }
            }
            sleep(Duration::from_secs(MONITORING_INTERVAL_SECS)).await;
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let tao::event::Event::MenuEvent {
                menu_id: tao::menu::MenuId(1),
                ..
            } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
