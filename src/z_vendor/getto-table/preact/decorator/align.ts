import { TableDataAlign, TableDataAlignStyle } from "../style"

export const decorateAlign =
    (aligns: readonly TableDataAlign[]): Decorator<TableDataAlignStyle> =>
    (style) => {
        const update = { ...style }
        aligns.forEach((align): true => {
            switch (align) {
                case "inherit":
                    return true

                case "top":
                case "middle":
                case "baseline":
                case "bottom":
                    update.vertical = align
                    return true

                case "left":
                case "center":
                case "right":
                case "numeric":
                    update.horizontal = align
                    return true
            }
        })
        return update
    }

interface Decorator<T> {
    (base: T): T
}
