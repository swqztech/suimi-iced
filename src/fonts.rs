use std::path::PathBuf;

/// 读取 Windows 常见中文字体，供 iced 加载。
pub fn load_system_cjk_font() -> Option<Vec<u8>> {
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| r"C:\Windows".to_string());
    let fonts_dir = PathBuf::from(windir).join("Fonts");
    for name in ["msyh.ttc", "msyhbd.ttc", "simhei.ttf", "simsun.ttc"] {
        let path = fonts_dir.join(name);
        if let Ok(data) = std::fs::read(&path) {
            return Some(data);
        }
    }
    None
}
