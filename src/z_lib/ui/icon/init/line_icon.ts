import { Icon } from "../data"

export function lnir(name: readonly string[]): Icon {
    return { toString: () => lni("lni", name) } as Icon
}
export function lnil(name: readonly string[]): Icon {
    return { toString: () => lni("lnil", name) } as Icon
}

function lni(type: string, name: readonly string[]): string {
    return [type, ...name.map((name) => `${type}-${name}`)].join(" ")
}
