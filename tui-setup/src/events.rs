use crossterm::event::KeyEvent;

use crate::installer::InstallerEvent;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Event {
    Key(KeyEvent),
    Tick,
    Installer(InstallerEvent),
}
