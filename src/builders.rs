use crate::main_app::{MainApp, ProcessWindow};

/// Build the main window
pub fn build_main_window(window: &mut MainApp) -> Result<(), nwg::NwgError> {
    nwg::Window::builder()
        .flags(
            nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE | nwg::WindowFlags::MINIMIZE_BOX,
        )
        .title("thstat")
        .size((300, 300))
        .build(&mut window.main_window)?;
    nwg::Button::builder()
        .parent(&window.main_window)
        .text("Scan Processes")
        .build(&mut window.scan_button)?;
    nwg::Button::builder()
        .parent(&window.main_window)
        .text("Hook Selected")
        .build(&mut window.hook_button)?;
    nwg::ListBox::builder()
        .parent(&window.main_window)
        .build(&mut window.process_list)?;
    nwg::GridLayout::builder()
        .parent(&window.main_window)
        .max_column(Some(2))
        .max_row(Some(4))
        .child(0, 0, &window.scan_button)
        .child(1, 0, &window.hook_button)
        .child_item(nwg::GridLayoutItem::new(&window.process_list, 0, 1, 2, 3))
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
