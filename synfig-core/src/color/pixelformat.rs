pub const PF_RGB: u16 = 0;
pub const PF_GRAY: u16 = 1 << 0; // If set, use one grayscale channel. If clear, use three channels for RGB
pub const PF_A: u16 = 1 << 1; // If set, include alpha channel
pub const PF_BGR: u16 = 1 << 2; // If set, reverse the order of the RGB channels
pub const PF_A_START: u16 = 1 << 3 | PF_A; // If set, alpha channel is before the color data. If clear, it is after.
pub const PF_A_PREMULT: u16 = 1 << 6 | PF_A; // If set, the encoded color channels are alpha-premulted
pub const PF_RAW_COLOR: u16 = 1 << 15 | PF_A; // If set, the data represents a raw Color data structure, and all other bits are ignored
