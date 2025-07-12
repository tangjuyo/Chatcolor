use pumpkin_util::text::color::NamedColor;

// Convertit NamedColor en code couleur Minecraft (ex: &a)
pub fn color_to_code(color: NamedColor) -> char {
    match color {
        NamedColor::Black => '0',
        NamedColor::DarkBlue => '1',
        NamedColor::DarkGreen => '2',
        NamedColor::DarkAqua => '3',
        NamedColor::DarkRed => '4',
        NamedColor::DarkPurple => '5',
        NamedColor::Gold => '6',
        NamedColor::Gray => '7',
        NamedColor::DarkGray => '8',
        NamedColor::Blue => '9',
        NamedColor::Green => 'a',
        NamedColor::Aqua => 'b',
        NamedColor::Red => 'c',
        NamedColor::LightPurple => 'd',
        NamedColor::Yellow => 'e',
        NamedColor::White => 'f',
    }
} 