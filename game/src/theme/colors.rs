use bevy::prelude::Color;

type Color50 = String;
type Color100 = String;
type Color200 = String;
type Color300 = String;
type Color400 = String;
type Color500 = String;
type Color600 = String;
type Color700 = String;
type Color800 = String;
type Color900 = String;
type Color950 = String;

type ColorTones = (
    Color50,
    Color100,
    Color200,
    Color300,
    Color400,
    Color500,
    Color600,
    Color700,
    Color800,
    Color900,
    Color950,
);

#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub slate: TailwindColor,
    pub gray: TailwindColor,
}

impl ColorTheme {
    pub fn new() -> Self {
        let slate = TailwindColor::new(
            "slate",
            (
                "f8fafc".to_string(),
                "f1f5f9".to_string(),
                "e2e8f0".to_string(),
                "cbd5e1".to_string(),
                "94a3b8".to_string(),
                "64748b".to_string(),
                "475569".to_string(),
                "334155".to_string(),
                "1e293b".to_string(),
                "0f172a".to_string(),
                "020617".to_string(),
            ),
        );

        let gray = TailwindColor::new(
            "gray",
            (
                "f9fafb".to_string(),
                "f3f4f6".to_string(),
                "e5e7eb".to_string(),
                "d1d5db".to_string(),
                "9ca3af".to_string(),
                "6b7280".to_string(),
                "4b5563".to_string(),
                "374151".to_string(),
                "1f2937".to_string(),
                "111827".to_string(),
                "030712".to_string(),
            ),
        );

        Self { slate, gray }
    }
}

#[derive(Debug, Clone)]
pub struct TailwindColor {
    pub name: String,
    pub tones: ColorTones,
}

impl TailwindColor {
    pub fn new(name: &str, tones: ColorTones) -> TailwindColor {
        TailwindColor {
            name: name.to_string(),
            tones,
        }
    }

    pub fn get_50(&self) -> Color {
        Color::hex(&self.tones.0).unwrap()
    }

    pub fn get_100(&self) -> Color {
        Color::hex(&self.tones.1).unwrap()
    }

    pub fn get_200(&self) -> Color {
        Color::hex(&self.tones.2).unwrap()
    }

    pub fn get_300(&self) -> Color {
        Color::hex(&self.tones.3).unwrap()
    }

    pub fn get_400(&self) -> Color {
        Color::hex(&self.tones.4).unwrap()
    }

    pub fn get_500(&self) -> Color {
        Color::hex(&self.tones.5).unwrap()
    }

    pub fn get_600(&self) -> Color {
        Color::hex(&self.tones.6).unwrap()
    }

    pub fn get_700(&self) -> Color {
        Color::hex(&self.tones.7).unwrap()
    }

    pub fn get_800(&self) -> Color {
        Color::hex(&self.tones.8).unwrap()
    }

    pub fn get_900(&self) -> Color {
        Color::hex(&self.tones.9).unwrap()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
