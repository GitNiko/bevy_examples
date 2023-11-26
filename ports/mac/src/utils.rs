use bevy::prelude::*;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn hex_to_vec4(hex: &str) -> Result<Vec4, &'static str> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 || hex.len() == 8 {
        let mut chars = hex.chars();
        let r = chars.next().and_then(|c| c.to_digit(16)).ok_or("Invalid hex character")? as f32;
        let r = r * 16.0 + chars.next().and_then(|c| c.to_digit(16)).ok_or("Invalid hex character")? as f32;
        let g = chars.next().and_then(|c| c.to_digit(16)).ok_or("Invalid hex character")? as f32;
        let g = g * 16.0 + chars.next().and_then(|c| c.to_digit(16)).ok_or("Invalid hex character")? as f32;
        let b = chars.next().and_then(|c| c.to_digit(16)).ok_or("Invalid hex character")? as f32;
        let b = b * 16.0 + chars.next().and_then(|c| c.to_digit(16)).ok_or("Invalid hex character")? as f32;
        let a = if let Some(c) = chars.next() {
            let a = c.to_digit(16).ok_or("Invalid hex character")? as f32;
            a * 16.0 + chars.next().and_then(|c| c.to_digit(16)).ok_or("Invalid hex character")? as f32
        } else {
            255.0
        };
        Ok(Vec4::new(r / 255.0, g / 255.0, b / 255.0, a / 255.0))
    } else {
        Err("Hex color should be 6 or 8 characters long")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_vec4() {
        assert_eq!(hex_to_vec4("#000000").unwrap(), Vec4::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(hex_to_vec4("#ffffff").unwrap(), Vec4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(hex_to_vec4("#ff0000").unwrap(), Vec4::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(hex_to_vec4("#00ff00").unwrap(), Vec4::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(hex_to_vec4("#0000ff").unwrap(), Vec4::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(hex_to_vec4("#ff00ff").unwrap(), Vec4::new(1.0, 0.0, 1.0, 1.0));
        assert_eq!(hex_to_vec4("#00ffff").unwrap(), Vec4::new(0.0, 1.0, 1.0, 1.0));
        assert_eq!(hex_to_vec4("#ffff00").unwrap(), Vec4::new(1.0, 1.0, 0.0, 1.0));
        assert_eq!(hex_to_vec4("#00000000").unwrap(), Vec4::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(hex_to_vec4("#ffffffff").unwrap(), Vec4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(hex_to_vec4("#ff0000ff").unwrap(), Vec4::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(hex_to_vec4("#00ff00ff").unwrap(), Vec4::new(0.0, 1.0, 0.0, 1.0));
    }
}
