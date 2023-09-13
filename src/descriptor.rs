use defmt::Format;

pub const COLUMN_COUNT: usize = 16;

pub const ROW_COUNT: usize = 5;

pub type LayoutGrid = [[LayoutKey; COLUMN_COUNT]; ROW_COUNT];

// Keyboard key usage ids that can be used in KeyboardReport
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Format)]
pub enum LayoutKey {
  KeyA = 0x04,
  KeyB = 0x05,
  KeyC = 0x06,
  KeyD = 0x07,
  KeyE = 0x08,
  KeyF = 0x09,

  CtrL = 0xe0,
  SftL = 0xe1,
  AltL = 0xe2,
  GuiL = 0xe3,
  CtrR = 0xe4,
  SftR = 0xe5,
  AltR = 0xe6,
  GuiR = 0xe7,
}

impl From<LayoutKey> for u8 {
  fn from(lk: LayoutKey) -> u8 {
    lk as u8
  }
}
