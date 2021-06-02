export type TableDataStyle = Readonly<{
    horizontalBorder: TableDataHorizontalBorderStyle
    align: TableDataAlignStyle
    className: TableDataClassName
}>
export type TableDataFullStyle = Readonly<{
    border: TableDataBorderStyle
    align: TableDataAlignStyle
    className: TableDataClassName
}>
export type TableDataRowStyle = Readonly<{
    className: TableDataClassName
}>

export type TableDataSticky =
    | Readonly<{ type: "none" }>
    | Readonly<{ type: "table" }>
    | Readonly<{ type: "header" }>
    | Readonly<{ type: "column"; column: number }>
    | Readonly<{ type: "cross"; column: number }>

export function inheritStyle(): TableDataStyle {
    return {
        horizontalBorder: { top: "inherit", bottom: "inherit" },
        align: { vertical: "inherit", horizontal: "inherit" },
        className: [],
    }
}
export function inheritVerticalBorderStyle(): TableDataVerticalBorderStyle {
    return { left: "inherit", right: "inherit" }
}
export function emptyRowStyle(): TableDataRowStyle {
    return {
        className: [],
    }
}

export function defaultHeaderStyle(): TableDataStyle {
    return {
        horizontalBorder: { top: "single", bottom: "double" },
        align: { vertical: "bottom", horizontal: "left" },
        className: [],
    }
}
export function defaultSummaryStyle(): TableDataStyle {
    return {
        horizontalBorder: { top: "inherit", bottom: "double" },
        align: { vertical: "top", horizontal: "left" },
        className: [],
    }
}
export function defaultColumnStyle(): TableDataStyle {
    return {
        horizontalBorder: { top: "none", bottom: "single" },
        align: { vertical: "top", horizontal: "left" },
        className: [],
    }
}
export function defaultFooterStyle(): TableDataStyle {
    return {
        horizontalBorder: { top: "double", bottom: "single" },
        align: { vertical: "top", horizontal: "left" },
        className: [],
    }
}
export function defaultVerticalBorderStyle(): TableDataVerticalBorderStyle {
    return { left: "none", right: "none" }
}

export function baseGroupStyle(style: TableDataStyle): TableDataStyle {
    return { ...style, horizontalBorder: { ...style.horizontalBorder, bottom: "single" } }
}
export function baseGroupMemberStyle(style: TableDataStyle): TableDataStyle {
    return { ...style, horizontalBorder: { ...style.horizontalBorder, top: "inherit" } }
}

export function extendStyle({
    base,
    style,
}: Readonly<{
    base: TableDataStyle
    style: TableDataStyle
}>): TableDataStyle {
    return {
        horizontalBorder: extendHorizontalBorderStyle({
            base: base.horizontalBorder,
            style: style.horizontalBorder,
        }),
        align: extendAlign({ base: base.align, style: style.align }),
        className: extendClassName({ base: base.className, style: style.className }),
    }
}

export function overrideBorderBottom(
    base: TableDataFullStyle,
    border: TableDataHorizontalBorderStyle
): TableDataFullStyle {
    return overrideBorderBottomTo(base, border.bottom)
}

export function overrideBorderBottomToNone(base: TableDataFullStyle): TableDataFullStyle {
    return overrideBorderBottomTo(base, "none")
}
export function overrideBorderBottomToSingle(base: TableDataFullStyle): TableDataFullStyle {
    return overrideBorderBottomTo(base, "single")
}
function overrideBorderBottomTo(
    base: TableDataFullStyle,
    border: TableDataBorderClass
): TableDataFullStyle {
    return {
        ...base,
        border: {
            ...base.border,
            horizontal: {
                ...base.border.horizontal,
                bottom: border,
            },
        },
    }
}

export function mergeVerticalBorder(
    style: TableDataStyle,
    vertical: TableDataVerticalBorderStyle
): TableDataFullStyle {
    return {
        ...style,
        border: mergeBorderStyle(style.horizontalBorder, vertical),
    }
}

export function treePaddingStyle(
    base: TableDataStyle,
    horizontal: TableDataHorizontalBorderStyle,
    vertical: TableDataVerticalBorderStyle
): TableDataFullStyle {
    return {
        ...base,
        border: {
            horizontal: extendHorizontalBorderStyle({
                base: base.horizontalBorder,
                style: horizontal,
            }),
            vertical,
        },
    }
}

export type TableDataVerticalBorder =
    | "left"
    | "leftDouble"
    | "leftNone"
    | "right"
    | "rightDouble"
    | "rightNone"
export type TableDataHorizontalBorder =
    | "top"
    | "topDouble"
    | "topNone"
    | "bottom"
    | "bottomDouble"
    | "bottomNone"

export type TableDataBorderStyle = Readonly<{
    horizontal: TableDataHorizontalBorderStyle
    vertical: TableDataVerticalBorderStyle
}>
export type TableDataVerticalBorderStyle = Readonly<{
    left: TableDataBorderClass
    right: TableDataBorderClass
}>
export type TableDataHorizontalBorderStyle = Readonly<{
    top: TableDataBorderClass
    bottom: TableDataBorderClass
}>

export type TableDataBorderClass = "none" | "single" | "double" | "inherit"

function extendHorizontalBorderStyle({
    base,
    style,
}: Readonly<{
    base: TableDataHorizontalBorderStyle
    style: TableDataHorizontalBorderStyle
}>): TableDataHorizontalBorderStyle {
    return {
        top: inherit({ base: base.top, style: style.top }),
        bottom: inherit({ base: base.bottom, style: style.bottom }),
    }

    function inherit({
        base,
        style,
    }: Readonly<{ base: TableDataBorderClass; style: TableDataBorderClass }>) {
        return style === "inherit" ? base : style
    }
}

function mergeBorderStyle(
    horizontal: TableDataHorizontalBorderStyle,
    vertical: TableDataVerticalBorderStyle
): TableDataBorderStyle {
    return { horizontal, vertical }
}

export type TableDataAlignStyle = Readonly<{
    vertical: TableDataVerticalAlign
    horizontal: TableDataHorizontalAlign
}>
export type TableDataAlign = TableDataVerticalAlign | TableDataHorizontalAlign
type TableDataVerticalAlign = "top" | "middle" | "baseline" | "bottom" | TableDataInheritAlign
type TableDataHorizontalAlign = "left" | "center" | "right" | "numeric" | TableDataInheritAlign
type TableDataInheritAlign = "inherit"

function extendAlign({
    base,
    style,
}: Readonly<{
    base: TableDataAlignStyle
    style: TableDataAlignStyle
}>): TableDataAlignStyle {
    return {
        vertical: inheritVertical({ base: base.vertical, style: style.vertical }),
        horizontal: inheritHorizontal({ base: base.horizontal, style: style.horizontal }),
    }

    function inheritVertical({
        base,
        style,
    }: Readonly<{ base: TableDataVerticalAlign; style: TableDataVerticalAlign }>) {
        return style === "inherit" ? base : style
    }
    function inheritHorizontal({
        base,
        style,
    }: Readonly<{ base: TableDataHorizontalAlign; style: TableDataHorizontalAlign }>) {
        return style === "inherit" ? base : style
    }
}

export type TableDataClassName = string[]

function extendClassName({
    base,
    style,
}: Readonly<{
    base: TableDataClassName
    style: TableDataClassName
}>): TableDataClassName {
    return [...base, ...style]
}

export type TableDataVisibleType = "normal" | "always"
