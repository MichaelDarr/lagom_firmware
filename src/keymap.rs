use super::descriptor::{
    LayoutGrid,
    LayoutKey::*,
};

pub const LAYOUT: LayoutGrid = [
    [Escp, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, Mnus, Plus, None, Bksp, PgUp],
    [Tabb, KeyQ, KeyW, KeyE, KeyR, KeyT, KeyY, KeyU, KeyI, KeyO, KeyP, BktL, BktR, Bslh, PgDn, None],
    [Grve, KeyA, KeyS, KeyD, KeyF, KeyG, KeyH, KeyJ, KeyK, KeyL, Semi, Quot, None, Entr, Home, None],
    [SftL, None, KeyZ, KeyX, KeyC, KeyV, KeyB, KeyN, KeyM, Coma, Perd, Slsh, SftR, ArwU, Endd, None],
    [CtrL, GuiL, AltL, None, None, None, Spce, None, None, None, AltR, CtrR, ArwL, ArwD, ArwR, None],
];
