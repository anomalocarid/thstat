// All the boilerplate code to generate windows and controls goes in this file
use crate::main_app::{MainApp, ProcessWindow};
use std::time::Duration;

/// Build the main window
pub fn build_main_window(window: &mut MainApp) -> Result<(), nwg::NwgError> {
    nwg::Window::builder()
        .flags(
            nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE | nwg::WindowFlags::MINIMIZE_BOX,
        )
        .title("thstat")
        .size((400, 600))
        .build(&mut window.main_window)?;
    nwg::AnimationTimer::builder()
        .parent(&window.main_window)
        .interval(Duration::from_millis(100))
        .lifetime(None)
        .max_tick(None)
        .active(true)
        .build(&mut window.update_timer)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("Scanning for running game...")
        .build(&mut window.status_label)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("Hi-Score: ")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.hiscore_label)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("0")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.hiscore_value)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("Score: ")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.score_label)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("0")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.score_value)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("Power: ")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.power_label)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("0.00")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.power_value)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("Lives: ")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.lives_label)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("0")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.lives_value)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("Bombs: ")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.bombs_label)?;
    nwg::Label::builder()
        .parent(&window.main_window)
        .text("0")
        .h_align(nwg::HTextAlign::Right)
        .build(&mut window.bombs_value)?;
    nwg::GridLayout::builder()
        .parent(&window.main_window)
        .max_column(Some(2))
        .max_row(Some(6))
        .child_item(nwg::GridLayoutItem::new(&window.status_label, 0, 0, 2, 1))
        .child(0, 1, &window.hiscore_label)
        .child(1, 1, &window.hiscore_value)
        .child(0, 2, &window.score_label)
        .child(1, 2, &window.score_value)
        .child(0, 3, &window.power_label)
        .child(1, 3, &window.power_value)
        .child(0, 4, &window.lives_label)
        .child(1, 4, &window.lives_value)
        .child(0, 5, &window.bombs_label)
        .child(1, 5, &window.bombs_value)
        .build(&mut window.layout)
}

/// Build the process window
pub fn build_process_window(
    parent: &nwg::Window,
    window: &mut ProcessWindow,
) -> Result<(), nwg::NwgError> {
    let (px, py) = parent.position();
    let (pw, _ph) = parent.size();
    nwg::Window::builder()
        .parent(Some(parent))
        .flags(nwg::WindowFlags::WINDOW)
        .title("Process View")
        .position((8 + px + pw as i32, py))
        .size((500, 200))
        .build(&mut window.handle)?;

    nwg::Label::builder()
        .parent(&window.handle)
        .text("Process name: ")
        .build(&mut window.name_label)?;
    nwg::Label::builder()
        .parent(&window.handle)
        .text("")
        .build(&mut window.name_value)?;
    nwg::Label::builder()
        .parent(&window.handle)
        .text("Process ID: ")
        .build(&mut window.id_label)?;
    nwg::Label::builder()
        .parent(&window.handle)
        .text("")
        .build(&mut window.id_value)?;

    nwg::GridLayout::builder()
        .parent(&window.handle)
        .max_column(Some(2))
        .child(0, 0, &window.name_label)
        .child(1, 0, &window.name_value)
        .child(0, 1, &window.id_label)
        .child(1, 1, &window.id_value)
        .build(&mut window.layout)
}

/// Build the menu for the main window
pub fn build_menu(data: &mut MainApp) -> Result<(), nwg::NwgError> {
    nwg::Menu::builder()
        .text("&File")
        .disabled(false)
        .parent(&data.main_window.handle)
        .build(&mut data.menu.file)?;
    nwg::MenuItem::builder()
        .text("E&xit")
        .disabled(false)
        .parent(&data.menu.file)
        .build(&mut data.menu.file_exit)?;
    nwg::Menu::builder()
        .text("&View")
        .disabled(false)
        .parent(&data.main_window.handle)
        .build(&mut data.menu.view)?;
    nwg::MenuItem::builder()
        .text("Process window")
        .disabled(false)
        .parent(&data.menu.view)
        .build(&mut data.menu.view_process)?;
    nwg::Menu::builder()
        .text("&Help")
        .disabled(false)
        .parent(&data.main_window.handle)
        .build(&mut data.menu.help)?;
    nwg::MenuItem::builder()
        .text("&About")
        .disabled(false)
        .parent(&data.menu.help)
        .build(&mut data.menu.help_about)
}
