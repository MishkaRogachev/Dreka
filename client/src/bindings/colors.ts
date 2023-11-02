export enum EntityColor {
    Slate = "Slate",
    Emerald = "Emerald",
    Teal = "Teal",
    Cyan = "Cyan",
    Sky = "Sky",
    Blue = "Blue",
    Indigo = "Indigo",
    Violet = "Violet"
}

export function toColorCode(color: string): string {
    switch (color) {
    case EntityColor.Slate:
        return "#94a3b8"
    case EntityColor.Emerald:
        return "#10b981"
    case EntityColor.Teal:
        return "#2dd4bf"
    case EntityColor.Cyan:
        return "#22d3ee"
    case EntityColor.Sky:
        return "#38bdf8"
    case EntityColor.Blue:
        return "#60a5fa"
    case EntityColor.Indigo:
        return "#818cf8"
    case EntityColor.Violet:
        return "#a78bfa"
    default:
        return ""
    }
}