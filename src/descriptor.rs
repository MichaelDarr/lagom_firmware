use defmt::Format;

pub const COLUMN_COUNT: usize = 16;

pub const ROW_COUNT: usize = 5;

pub type LayoutGrid = [[LayoutKey; COLUMN_COUNT]; ROW_COUNT];

// Keyboard key usage ids that can be used in KeyboardReport
//
// Values sourced from chromium:
// https://chromium.googlesource.com/chromium/src/+/dff16958029d9a8fb9004351f72e961ed4143e83/ui/events/keycodes/dom/keycode_converter_data.inc#319
//
// All `07`-prefixed keys ("OSRight" and lower) are represented in this enum.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Format)]
pub enum LayoutKey {
  // USB reserved, treated as unassigned
  None = 0x00,
  // USB error roll over
  URol = 0x01,
  // USB post fail
  UPst = 0x02,
  // USB error undefined
  UErr = 0x03,

  KeyA = 0x04,
  KeyB = 0x05,
  KeyC = 0x06,
  KeyD = 0x07,
  KeyE = 0x08,
  KeyF = 0x09,
  KeyG = 0x0a,
  KeyH = 0x0b,
  KeyI = 0x0c,
  KeyJ = 0x0d,
  KeyK = 0x0e,
  KeyL = 0x0f,
  KeyM = 0x10,
  KeyN = 0x11,
  KeyO = 0x12,
  KeyP = 0x13,
  KeyQ = 0x14,
  KeyR = 0x15,
  KeyS = 0x16,
  KeyT = 0x17,
  KeyU = 0x18,
  KeyV = 0x19,
  KeyW = 0x1a,
  KeyX = 0x1b,
  KeyY = 0x1c,
  KeyZ = 0x1d,

  Key1 = 0x1e,
  Key2 = 0x1f,
  Key3 = 0x20,
  Key4 = 0x21,
  Key5 = 0x22,
  Key6 = 0x23,
  Key7 = 0x24,
  Key8 = 0x25,
  Key9 = 0x26,
  Key0 = 0x27,

  Entr = 0x28,
  Escp = 0x29,
  Bksp = 0x2a,
  Tabb = 0x2b,
  Spce = 0x2c,
  Mnus = 0x2d,
  Plus = 0x2e,
  BktL = 0x2f,
  BktR = 0x30,
  Bslh = 0x31,
  IHsh = 0x32,
  Semi = 0x33,
  Quot = 0x34,
  Grve = 0x35,
  Coma = 0x36,
  Perd = 0x37,
  Slsh = 0x38,
  CpLk = 0x39,

  Fnc1 = 0x3a,
  Fnc2 = 0x3b,
  Fnc3 = 0x3c,
  Fnc4 = 0x3d,
  Fnc5 = 0x3e,
  Fnc6 = 0x3f,
  Fnc7 = 0x40,
  Fnc8 = 0x41,
  Fnc9 = 0x42,
  Fn10 = 0x43,
  Fn11 = 0x44,
  Fn12 = 0x45,

  Prnt = 0x46,
  ScLk = 0x47,
  Paws = 0x48,
  Inst = 0x49,
  Home = 0x4a,
  PgUp = 0x4b,
  Dlte = 0x4c,
  Endd = 0x4d,
  PgDn = 0x4e,
  ArwR = 0x4f,
  ArwL = 0x50,
  ArwD = 0x51,
  ArwU = 0x52,
  NmLk = 0x53,

  NDiv = 0x54,
  NMlt = 0x55,
  NSub = 0x56,
  NAdd = 0x57,
  NEnt = 0x58,
  NNm1 = 0x59,
  NNm2 = 0x5a,
  NNm3 = 0x5b,
  NNm4 = 0x5c,
  NNm5 = 0x5d,
  NNm6 = 0x5e,
  NNm7 = 0x5f,
  NNm8 = 0x60,
  NNm9 = 0x61,
  NNm0 = 0x62,
  NDcm = 0x63,

  IBsl = 0x64,
  CtxM = 0x65,
  Powr = 0x66,
  NpEq = 0x67,

  Fn13 = 0x68,
  Fn14 = 0x69,
  Fn15 = 0x6a,
  Fn16 = 0x6b,
  Fn17 = 0x6c,
  Fn18 = 0x6d,
  Fn19 = 0x6e,
  Fn20 = 0x6f,
  Fn21 = 0x70,
  Fn22 = 0x71,
  Fn23 = 0x72,
  Fn24 = 0x73,

  Open = 0x74,
  Help = 0x75,
  Slct = 0x77,
  Agyn = 0x79,
  Undo = 0x7a,
  Cutt = 0x7b,
  Copy = 0x7c,
  Pste = 0x7d,
  Find = 0x7e,
  VlMt = 0x7f,
  VlUp = 0x80,
  VlDn = 0x81,

  NCma = 0x85,

  // Language & international keys
  Brzl = 0x87,
  Kana = 0x88,
  IYen = 0x89,
  Cnvt = 0x8a,
  NCnv = 0x8b,
  Lng1 = 0x90,
  Lng2 = 0x91,
  Lng3 = 0x92,
  Lng4 = 0x93,
  Lng5 = 0x94,

  Abrt = 0x9b,
  Prps = 0xa3,

  // Numpad parens and backspace
  NPnL = 0xb6,
  NPnR = 0xb7,
  NBsp = 0xbb,

  // Numpad memory
  NMSr = 0xd0,
  NMRc = 0xd1,
  NMCl = 0xd2,
  NMAd = 0xd3,
  NMSb = 0xd4,

  // Numpad sign change
  NSCh = 0xd7,
  // Clear numpad
  NClr = 0xd8,
  // Clear numpad entry
  NClE = 0xd9,

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
