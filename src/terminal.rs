use termion;
pub struct TerminalController {

}

impl TerminalController {

    pub fn clear() {
        print!("{}", termion::clear::All);
    }
}