export type LineIcon = Readonly<{ style: "regular" | "light"; name: string }> & { LineIcon: never }

export function lnir(name: string): LineIcon {
    return { style: "regular", name } as LineIcon
}
export function lnil(name: string): LineIcon {
    return { style: "light", name } as LineIcon
}

export function lniClass(icon: LineIcon): string {
    switch (icon.style) {
        case "regular":
            return `lnir lnir-${icon.name}`
        case "light":
            return `lnil lnil-${icon.name}`
    }
}
