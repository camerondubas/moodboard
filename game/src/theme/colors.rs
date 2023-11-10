use bevy::prelude::Color;

#[derive(Debug, Clone)]
pub struct ColorTones {
    pub color50: String,
    pub color100: String,
    pub color200: String,
    pub color300: String,
    pub color400: String,
    pub color500: String,
    pub color600: String,
    pub color700: String,
    pub color800: String,
    pub color900: String,
    pub color950: String,
}

impl ColorTones {
    pub fn from_str(
        color50: &str,
        color100: &str,
        color200: &str,
        color300: &str,
        color400: &str,
        color500: &str,
        color600: &str,
        color700: &str,
        color800: &str,
        color900: &str,
        color950: &str,
    ) -> Self {
        Self {
            color50: color50.to_string(),
            color100: color100.to_string(),
            color200: color200.to_string(),
            color300: color300.to_string(),
            color400: color400.to_string(),
            color500: color500.to_string(),
            color600: color600.to_string(),
            color700: color700.to_string(),
            color800: color800.to_string(),
            color900: color900.to_string(),
            color950: color950.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub slate: TailwindColor,
    pub gray: TailwindColor,
    pub zinc: TailwindColor,
    pub neutral: TailwindColor,
    pub stone: TailwindColor,
    pub red: TailwindColor,
    pub orange: TailwindColor,
    pub amber: TailwindColor,
    pub yellow: TailwindColor,
    pub lime: TailwindColor,
    pub green: TailwindColor,
    pub emerald: TailwindColor,
    pub teal: TailwindColor,
    pub cyan: TailwindColor,
    pub sky: TailwindColor,
    pub blue: TailwindColor,
    pub indigo: TailwindColor,
    pub violet: TailwindColor,
    pub purple: TailwindColor,
    pub fuchsia: TailwindColor,
    pub pink: TailwindColor,
    pub rose: TailwindColor,
}

impl ColorTheme {
    pub fn new() -> Self {
        let slate = TailwindColor::new(
            "slate",
            ColorTones::from_str(
                "f8fafc", "f1f5f9", "e2e8f0", "cbd5e1", "94a3b8", "64748b", "475569", "334155",
                "1e293b", "0f172a", "020617",
            ),
        );

        let gray = TailwindColor::new(
            "gray",
            ColorTones::from_str(
                "f9fafb", "f3f4f6", "e5e7eb", "d1d5db", "9ca3af", "6b7280", "4b5563", "374151",
                "1f2937", "111827", "030712",
            ),
        );

        let zinc = TailwindColor::new(
            "zinc",
            ColorTones::from_str(
                "fafafa", "f4f4f5", "e4e4e7", "d4d4d8", "a1a1aa", "71717a", "52525b", "3f3f46",
                "27272a", "18181b", "0a0a0b",
            ),
        );

        let neutral = TailwindColor::new(
            "neutral",
            ColorTones::from_str(
                "fafafa", "f5f5f5", "e5e5e5", "d4d4d4", "a3a3a3", "737373", "525252", "404040",
                "262626", "171717", "0a0a0a",
            ),
        );

        let stone = TailwindColor::new(
            "stone",
            ColorTones::from_str(
                "fafaf9", "f5f5f4", "e7e5e4", "d6d3d1", "a8a29e", "78716c", "57534e", "44403c",
                "292524", "1c1917", "0c0a09",
            ),
        );

        let red = TailwindColor::new(
            "red",
            ColorTones::from_str(
                "fef2f2", "fee2e2", "fecaca", "fca5a5", "f87171", "ef4444", "dc2626", "b91c1c",
                "991b1b", "7f1d1d", "450a0a",
            ),
        );

        let orange = TailwindColor::new(
            "orange",
            ColorTones::from_str(
                "fff7ed", "ffedd5", "fed7aa", "fdba74", "fb923c", "f97316", "ea580c", "c2410c",
                "9a3412", "7c2d12", "431407",
            ),
        );

        let amber = TailwindColor::new(
            "amber",
            ColorTones::from_str(
                "fffbeb", "fef3c7", "fde68a", "fcd34d", "fbbf24", "f59e0b", "d97706", "b45309",
                "92400e", "78350f", "451a03",
            ),
        );

        let yellow = TailwindColor::new(
            "yellow",
            ColorTones::from_str(
                "fefce8", "fef9c3", "fef08a", "fde047", "facc15", "eab308", "ca8a04", "a16207",
                "854d0e", "713f12", "422006",
            ),
        );

        let lime = TailwindColor::new(
            "lime",
            ColorTones::from_str(
                "f7fee7", "ecfccb", "d9f99d", "bef264", "a3e635", "84cc16", "65a30d", "4d7c0f",
                "3f6212", "365314", "1a2e05",
            ),
        );

        let green = TailwindColor::new(
            "green",
            ColorTones::from_str(
                "f0fdf4", "dcfce7", "bbf7d0", "86efac", "4ade80", "22c55e", "16a34a", "15803d",
                "166534", "14532d", "052e16",
            ),
        );
        let emerald = TailwindColor::new(
            "emerald",
            ColorTones::from_str(
                "ecfdf5", "d1fae5", "a7f3d0", "6ee7b7", "34d399", "10b981", "059669", "047857",
                "065f46", "064e3b", "022c22",
            ),
        );

        let teal = TailwindColor::new(
            "teal",
            ColorTones::from_str(
                "f0fdfa", "ccfbf1", "99f6e4", "5eead4", "2dd4bf", "14b8a6", "0d9488", "0f766e",
                "115e59", "134e4a", "042f2e",
            ),
        );

        let cyan = TailwindColor::new(
            "cyan",
            ColorTones::from_str(
                "ecfeff", "cffafe", "a5f3fc", "67e8f9", "22d3ee", "06b6d4", "0891b2", "0e7490",
                "155e75", "164e63", "083344",
            ),
        );

        let sky = TailwindColor::new(
            "sky",
            ColorTones::from_str(
                "f0f9ff", "e0f2fe", "bae6fd", "7dd3fc", "38bdf8", "0ea5e9", "0284c7", "0369a1",
                "075985", "0c4a6e", "083344",
            ),
        );

        let blue = TailwindColor::new(
            "blue",
            ColorTones::from_str(
                "eff6ff", "dbeafe", "bfdbfe", "93c5fd", "60a5fa", "3b82f6", "2563eb", "1d4ed8",
                "1e40af", "1e3a8a", "172554",
            ),
        );

        let indigo = TailwindColor::new(
            "indigo",
            ColorTones::from_str(
                "eef2ff", "e0e7ff", "c7d2fe", "a5b4fc", "818cf8", "6366f1", "4f46e5", "4338ca",
                "3730a3", "312e81", "1e1b4b",
            ),
        );

        let violet = TailwindColor::new(
            "violet",
            ColorTones::from_str(
                "f5f3ff", "ede9fe", "ddd6fe", "c4b5fd", "a78bfa", "8b5cf6", "7c3aed", "6d28d9",
                "5b21b6", "4c1d95", "2e1065",
            ),
        );

        let purple = TailwindColor::new(
            "purple",
            ColorTones::from_str(
                "faf5ff", "f3e8ff", "e9d5ff", "d8b4fe", "c084fc", "a855f7", "9333ea", "7e22ce",
                "6b21a8", "581c87", "3b0764",
            ),
        );

        let fuchsia = TailwindColor::new(
            "fuchsia",
            ColorTones::from_str(
                "fdf4ff", "fae8ff", "f5d0fe", "f0abfc", "e879f9", "d946ef", "c026d3", "a21caf",
                "86198f", "701a75", "4a044e",
            ),
        );
        let pink = TailwindColor::new(
            "pink",
            ColorTones::from_str(
                "fdf2f8", "fce7f3", "fbcfe8", "f9a8d4", "f472b6", "ec4899", "db2777", "be185d",
                "9d174d", "831843", "500724",
            ),
        );

        let rose = TailwindColor::new(
            "rose",
            ColorTones::from_str(
                "fff1f2", "ffe4e6", "fecdd3", "fda4af", "fb7185", "f43f5e", "e11d48", "be123c",
                "9f1239", "881337", "4c0519",
            ),
        );

        Self {
            slate,
            gray,
            zinc,
            neutral,
            stone,
            red,
            orange,
            amber,
            yellow,
            lime,
            green,
            emerald,
            teal,
            cyan,
            sky,
            blue,
            indigo,
            violet,
            purple,
            fuchsia,
            pink,
            rose,
        }
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
        Color::hex(&self.tones.color50).unwrap()
    }

    pub fn get_100(&self) -> Color {
        Color::hex(&self.tones.color100).unwrap()
    }

    pub fn get_200(&self) -> Color {
        Color::hex(&self.tones.color200).unwrap()
    }

    pub fn get_300(&self) -> Color {
        Color::hex(&self.tones.color300).unwrap()
    }

    pub fn get_400(&self) -> Color {
        Color::hex(&self.tones.color400).unwrap()
    }

    pub fn get_500(&self) -> Color {
        Color::hex(&self.tones.color500).unwrap()
    }

    pub fn get_600(&self) -> Color {
        Color::hex(&self.tones.color600).unwrap()
    }

    pub fn get_700(&self) -> Color {
        Color::hex(&self.tones.color700).unwrap()
    }

    pub fn get_800(&self) -> Color {
        Color::hex(&self.tones.color800).unwrap()
    }

    pub fn get_900(&self) -> Color {
        Color::hex(&self.tones.color900).unwrap()
    }

    pub fn get_950(&self) -> Color {
        Color::hex(&self.tones.color950).unwrap()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
