import {
    TableDataHorizontalBorder,
    TableDataHorizontalBorderStyle,
    TableDataVerticalBorder,
    TableDataVerticalBorderStyle,
} from "../style"

export const decorateVerticalBorder =
    (borders: readonly TableDataVerticalBorder[]): Decorator<TableDataVerticalBorderStyle> =>
    (style) => {
        const vertical = { ...style }
        borders.forEach((border): true => {
            switch (border) {
                case "left":
                    vertical.left = "single"
                    return true

                case "leftDouble":
                    vertical.left = "double"
                    return true

                case "leftNone":
                    vertical.left = "none"
                    return true

                case "right":
                    vertical.right = "single"
                    return true

                case "rightDouble":
                    vertical.right = "double"
                    return true

                case "rightNone":
                    vertical.right = "none"
                    return true
            }
        })
        return vertical
    }

export const decorateHorizontalBorder =
    (borders: readonly TableDataHorizontalBorder[]): Decorator<TableDataHorizontalBorderStyle> =>
    (style) => {
        const horizontal = { ...style }
        borders.forEach((border): true => {
            switch (border) {
                case "top":
                    horizontal.top = "single"
                    return true

                case "topDouble":
                    horizontal.top = "double"
                    return true

                case "topNone":
                    horizontal.top = "none"
                    return true

                case "bottom":
                    horizontal.bottom = "single"
                    return true

                case "bottomDouble":
                    horizontal.bottom = "double"
                    return true

                case "bottomNone":
                    horizontal.bottom = "none"
                    return true
            }
        })
        return horizontal
    }

interface Decorator<T> {
    (base: T): T
}
