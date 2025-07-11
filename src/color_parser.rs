use pumpkin_util::text::{color::NamedColor, TextComponent};
use pumpkin_util::text::color::{Color, RGBColor};

/// Parse Minecraft color codes from a string and convert them to a TextComponent
pub fn parse_color_codes(input: &str) -> TextComponent {
    if !input.contains('&') {
        // No color codes, return plain text
        return TextComponent::text(input.to_string());
    }

    let mut components = Vec::new();
    let mut current_text = String::new();
    let mut current_color = None;
    let mut current_format = None;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '&' {
            if let Some(&next_char) = chars.peek() {
                if let Some(color_code) = parse_color_code(next_char) {
                    // Add the current text to the result if not empty
                    if !current_text.is_empty() {
                        let mut component = TextComponent::text(current_text.clone());
                        if let Some(color) = current_color {
                            component = component.color_named(color);
                        }
                        if let Some(format) = current_format {
                            component = apply_format_code(component, format);
                        }
                        components.push(component);
                        current_text.clear();
                    }
                    
                    // Skip the color code character
                    chars.next();
                    
                    // Set the new color
                    current_color = Some(color_code);
                } else if let Some(format_code) = parse_format_code(next_char) {
                    // Add the current text to the result if not empty
                    if !current_text.is_empty() {
                        let mut component = TextComponent::text(current_text.clone());
                        if let Some(color) = current_color {
                            component = component.color_named(color);
                        }
                        if let Some(format) = current_format {
                            component = apply_format_code(component, format);
                        }
                        components.push(component);
                        current_text.clear();
                    }
                    
                    // Skip the format code character
                    chars.next();
                    
                    // Set the new format
                    current_format = Some(format_code);
                } else if next_char == 'r' {
                    // Reset code - add current text and reset
                    if !current_text.is_empty() {
                        let mut component = TextComponent::text(current_text.clone());
                        if let Some(color) = current_color {
                            component = component.color_named(color);
                        }
                        if let Some(format) = current_format {
                            component = apply_format_code(component, format);
                        }
                        components.push(component);
                        current_text.clear();
                    }
                    current_color = None;
                    current_format = None;
                    chars.next();
                } else {
                    // Invalid code, treat as literal &
                    current_text.push('&');
                }
            } else {
                // & at end of string, treat as literal
                current_text.push('&');
            }
        } else {
            // Regular character, add to current text
            current_text.push(ch);
        }
    }

    // Add the final text if not empty
    if !current_text.is_empty() {
        let mut component = TextComponent::text(current_text);
        if let Some(color) = current_color {
            component = component.color_named(color);
        }
        if let Some(format) = current_format {
            component = apply_format_code(component, format);
        }
        components.push(component);
    }

    // Build the final result
    if components.is_empty() {
        TextComponent::text("")
    } else if components.len() == 1 {
        components.into_iter().next().unwrap()
    } else {
        let mut iter = components.into_iter();
        let mut result = iter.next().unwrap();
        for component in iter {
            result = result.add_child(component);
        }
        result
    }
}

/// Parse a color code character and return the corresponding NamedColor
fn parse_color_code(ch: char) -> Option<NamedColor> {
    match ch {
        '0' => Some(NamedColor::Black),
        '1' => Some(NamedColor::DarkBlue),
        '2' => Some(NamedColor::DarkGreen),
        '3' => Some(NamedColor::DarkAqua),
        '4' => Some(NamedColor::DarkRed),
        '5' => Some(NamedColor::DarkPurple),
        '6' => Some(NamedColor::Gold),
        '7' => Some(NamedColor::Gray),
        '8' => Some(NamedColor::DarkGray),
        '9' => Some(NamedColor::Blue),
        'a' => Some(NamedColor::Green),
        'b' => Some(NamedColor::Aqua),
        'c' => Some(NamedColor::Red),
        'd' => Some(NamedColor::LightPurple),
        'e' => Some(NamedColor::Yellow),
        'f' => Some(NamedColor::White),
        _ => None,
    }
}

/// Parse a format code character and return the corresponding format type
#[derive(Debug, Clone, Copy)]
enum FormatCode {
    Bold,
    Underlined,
    Italic,
    Strikethrough,
    Obfuscated,
}

fn parse_format_code(ch: char) -> Option<FormatCode> {
    match ch {
        'l' => Some(FormatCode::Bold),
        'n' => Some(FormatCode::Underlined),
        'o' => Some(FormatCode::Italic),
        'm' => Some(FormatCode::Strikethrough),
        'k' => Some(FormatCode::Obfuscated),
        _ => None,
    }
}

/// Apply a format code to a TextComponent
fn apply_format_code(component: TextComponent, format: FormatCode) -> TextComponent {
    match format {
        FormatCode::Bold => component.bold(),
        FormatCode::Underlined => component.underlined(),
        FormatCode::Italic => component.italic(),
        FormatCode::Strikethrough => component.strikethrough(),
        FormatCode::Obfuscated => component.obfuscated(),
    }
}

/// Applique un gradient arc-en-ciel lettre par lettre sur le texte.
pub fn apply_rainbow_gradient(text: &str) -> TextComponent {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len().max(1);
    let mut component = TextComponent::text("");
    for (i, c) in chars.iter().enumerate() {
        // HSV: H varie de 0 à 360 sur le texte
        let hue = (i as f32) / (len as f32);
        let rgb = hsv_to_rgb(hue, 1.0, 1.0);
        let color = Color::Rgb(RGBColor::new(rgb.0, rgb.1, rgb.2));
        let letter = TextComponent::text(c.to_string()).color(color);
        component = component.add_child(letter);
    }
    component
}

/// Applique un gradient "feu" (jaune -> orange -> rouge) lettre par lettre sur le texte.
pub fn apply_fire_gradient(text: &str) -> TextComponent {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len().max(1);
    let mut component = TextComponent::text("");
    for (i, c) in chars.iter().enumerate() {
        // Interpolation: jaune (#FFFF00) -> orange (#FF8000) -> rouge (#FF0000)
        let t = (i as f32) / (len as f32);
        let rgb = if t < 0.5 {
            lerp_rgb((255,255,0), (255,128,0), t*2.0)
        } else {
            lerp_rgb((255,128,0), (255,0,0), (t-0.5)*2.0)
        };
        let color = Color::Rgb(RGBColor::new(rgb.0, rgb.1, rgb.2));
        let letter = TextComponent::text(c.to_string()).color(color);
        component = component.add_child(letter);
    }
    component
}

// Utilitaires
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h * 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    (
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}

fn lerp_rgb(a: (u8,u8,u8), b: (u8,u8,u8), t: f32) -> (u8,u8,u8) {
    let t = t.clamp(0.0, 1.0);
    (
        (a.0 as f32 + (b.0 as f32 - a.0 as f32) * t) as u8,
        (a.1 as f32 + (b.1 as f32 - a.1 as f32) * t) as u8,
        (a.2 as f32 + (b.2 as f32 - a.2 as f32) * t) as u8,
    )
}

fn rgb_to_hex((r,g,b): (u8,u8,u8)) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color_codes() {
        // Test basic color codes
        let result = parse_color_codes("&aHello &bWorld!");
        assert!(!result.get_text().is_empty());
        
        // Test format codes
        let result = parse_color_codes("&lBold &nUnderlined");
        assert!(!result.get_text().is_empty());
        
        // Test reset code
        let result = parse_color_codes("&aColored &rNormal");
        assert!(!result.get_text().is_empty());
        
        // Test invalid codes
        let result = parse_color_codes("&xInvalid");
        assert!(!result.get_text().is_empty());
        
        // Test no color codes
        let result = parse_color_codes("Plain text");
        assert_eq!(result.get_text(), "Plain text");
    }

    #[test]
    fn test_parse_color_code() {
        assert_eq!(parse_color_code('a'), Some(NamedColor::Green));
        assert_eq!(parse_color_code('b'), Some(NamedColor::Aqua));
        assert_eq!(parse_color_code('c'), Some(NamedColor::Red));
        assert_eq!(parse_color_code('x'), None);
    }

    #[test]
    fn test_parse_format_code() {
        assert_eq!(parse_format_code('l'), Some(FormatCode::Bold));
        assert_eq!(parse_format_code('n'), Some(FormatCode::Underlined));
        assert_eq!(parse_format_code('o'), Some(FormatCode::Italic));
        assert_eq!(parse_format_code('x'), None);
    }
} 