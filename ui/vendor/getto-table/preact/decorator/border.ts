import {
    TableDataHorizontalBorder,
    TableDataHorizontalBorderStyle,
    TableDataVerticalBorder,
    TableDataVerticalBorderStyle,
} from "../style"

export const decorateVerticalBorder = (
    borders: TableDataVerticalBorder[]
): Decorator<TableDataVerticalBorderStyle> => (style) => {
    const vertical = { ...style }
    borders.forEach((border) => {
        switch (border) {
            case "left":
                vertical.left = "single"
                break
            case "leftDouble":
                vertical.left = "double"
                break
            case "leftNone":
                vertical.left = "none"
                break

            case "right":
                vertical.right = "single"
                break
            case "rightDouble":
                vertical.right = "double"
                break
            case "rightNone":
                vertical.right = "none"
                break

            default:
                assertNever(border)
        }
    })
    return vertical
}

export const decorateHorizontalBorder = (
    borders: TableDataHorizontalBorder[]
): Decorator<TableDataHorizontalBorderStyle> => (style) => {
    const horizontal = { ...style }
    borders.forEach((border) => {
        switch (border) {
            case "top":
                horizontal.top = "single"
                break
            case "topDouble":
                horizontal.top = "double"
                break
            case "topNone":
                horizontal.top = "none"
                break

            case "bottom":
                horizontal.bottom = "single"
                break
            case "bottomDouble":
                horizontal.bottom = "double"
                break
            case "bottomNone":
                horizontal.bottom = "none"
                break

            default:
                assertNever(border)
        }
    })
    return horizontal
}

function assertNever(_: never): never {
    throw new Error("NEVER")
}

interface Decorator<T> {
    (base: T): T
}
