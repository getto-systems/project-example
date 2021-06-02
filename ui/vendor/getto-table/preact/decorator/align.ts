import { TableDataAlign, TableDataAlignStyle } from "../style"

export const decorateAlign = (aligns: TableDataAlign[]): Decorator<TableDataAlignStyle> => (style) => {
    const update = { ...style }
    aligns.forEach((align) => {
        switch (align) {
            case "inherit":
                break
            case "top":
            case "middle":
            case "baseline":
            case "bottom":
                update.vertical = align
                break
            case "left":
            case "center":
            case "right":
            case "numeric":
                update.horizontal = align
                break
            default:
                assertNever(align)
        }
    })
    return update
}

function assertNever(_: never): never {
    throw new Error("NEVER")
}

interface Decorator<T> {
    (base: T): T
}
