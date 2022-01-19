export type LineIcon = Readonly<{ name: string }> & { LineIcon: never }

export function lnir(name: string): LineIcon {
    return { name } as LineIcon
}

export function lniClass(icon: LineIcon): string {
    return `lnir lnir-${icon.name}`
}
