use super::descriptor::{
    Keyboard,
    RotaryEncoderKeys,
    LayoutKey::*,
};

pub const KEYBOARD: Keyboard = Keyboard {
    layout: [
        [Escp, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, Mnus, Plus, None, Bksp, Home],
        [Tabb, KeyQ, KeyW, KeyE, KeyR, KeyT, KeyY, KeyU, KeyI, KeyO, KeyP, BktL, BktR, Bslh, Endd, VlMt],
        [Grve, KeyA, KeyS, KeyD, KeyF, KeyG, KeyH, KeyJ, KeyK, KeyL, Semi, Quot, None, Entr, PgUp, Dlte],
        [SftL, None, KeyZ, KeyX, KeyC, KeyV, KeyB, KeyN, KeyM, Coma, Perd, Slsh, SftR, ArwU, PgDn, None],
        [CtrL, GuiL, AltL, None, None, None, Spce, None, None, None, AltR, CtrR, ArwL, ArwD, ArwR, None],
    ],
    left_rotary_encoder: RotaryEncoderKeys {
        clockwise: Agyn,
        counter_clockwise: Undo,
    },
    right_rotary_encoder: RotaryEncoderKeys {
        clockwise: Kana,
        counter_clockwise: Cnvt,
    },
};
