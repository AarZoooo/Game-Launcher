use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone, Copy)]
pub enum PlaceholderKind {
    Vertical,
    Horizontal,
}

pub fn placeholder_data_url(title: &str, kind: PlaceholderKind) -> String {
    let (width, height) = match kind {
        PlaceholderKind::Vertical => (600, 900),
        PlaceholderKind::Horizontal => (1280, 720),
    };

    let safe_title = if title.trim().is_empty() {
        "Game"
    } else {
        title.trim()
    };

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
<defs>
  <linearGradient id="bg" x1="0" x2="1" y1="0" y2="1">
    <stop offset="0%" stop-color="#222b35"/>
    <stop offset="100%" stop-color="#39495b"/>
  </linearGradient>
</defs>
<rect width="{width}" height="{height}" fill="url(#bg)"/>
<circle cx="{circle_x}" cy="{circle_y}" r="{circle_r}" fill="rgba(255,255,255,0.08)"/>
<text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle"
      font-family="Segoe UI, Arial, sans-serif" font-size="{font_size}" font-weight="700" fill="#f3f5f7">{safe_title}</text>
</svg>"##,
        circle_x = width - (width / 6),
        circle_y = height / 5,
        circle_r = width / 9,
        font_size = (width.min(height) / 12).max(28),
    );

    format!("data:image/svg+xml;base64,{}", STANDARD.encode(svg))
}

#[cfg(test)]
mod tests {
    use super::{PlaceholderKind, placeholder_data_url};

    #[test]
    fn creates_svg_placeholder_data_url() {
        let placeholder = placeholder_data_url("Satisfactory", PlaceholderKind::Vertical);
        assert!(placeholder.starts_with("data:image/svg+xml;base64,"));
    }
}
