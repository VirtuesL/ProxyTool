// A4 at 300 DPI: 2480 x 3508 pixels
pub(crate) const A4_WIDTH: usize = 2480;
pub(crate) const A4_HEIGHT: usize = 3508;

// MTG card size at 300 DPI (63mm x 88mm)
pub(crate) const CARD_WIDTH: usize = 744;
pub(crate) const CARD_HEIGHT: usize = 1039;

// Grid layout: 3x3 cards per page
pub(crate) const CARDS_PER_ROW: usize = 3;
pub(crate) const CARDS_PER_COL: usize = 3;
pub(crate) const CARDS_PER_PAGE: usize = CARDS_PER_ROW * CARDS_PER_COL;

pub(crate) const MARGIN_X: usize = (A4_WIDTH - (CARD_WIDTH * CARDS_PER_ROW)) / 2;
pub(crate) const MARGIN_Y: usize = (A4_HEIGHT - (CARD_HEIGHT * CARDS_PER_COL)) / 2;
