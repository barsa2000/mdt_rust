#[derive(Debug, Clone, Copy)]
pub enum HeadDirection {
    Right,
    Left,
    None,
}
#[derive(Debug, Clone)]
pub struct MDTRule {
    pub current_state: String,
    pub read_symbol: char,
    pub next_state: String,
    pub write_symbol: char,
    pub direction: HeadDirection,
}
