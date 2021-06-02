import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent, VNodeKey } from "../common"

import {
    TableDataColumnRow,
    TableDataFooterRow,
    TableDataHeader,
    TableDataHeaderRow,
    TableDataSummary,
    TableDataSummaryRow,
    TableDataColumnTree,
    TableDataColumn,
    TableDataColumnSimple,
    TableDataColumnExpansion,
} from "../../../getto-table/preact/core"
import {
    overrideBorderBottomToNone,
    overrideBorderBottomToSingle,
    TableDataAlign,
    TableDataAlignStyle,
    TableDataBorderClass,
    TableDataBorderStyle,
    TableDataClassName,
    TableDataFullStyle,
    TableDataSticky,
} from "../../../getto-table/preact/style"

export interface SortLink {
    (key: SortKey): { (content: VNodeContent): VNode }
}
export type Sort = Readonly<{
    key: SortKey
    order: SortOrder
    href: { (query: SortQuery): SortHref }
    sign: SortSign
}>
export type SortKey = VNodeKey
export type SortOrder = "normal" | "reverse"
export type SortQuery = Readonly<{
    key: SortKey
    order: SortOrder
}>
export type SortHref = string
export type SortSign = Readonly<{
    normal: VNodeContent
    reverse: VNodeContent
}>
export function sortLink(sort: Sort): SortLink {
    return (key) => (content) =>
        html`<a href=${sort.href(sortQuery(key))}>${content} ${sortSign(key)}</a>`

    function sortQuery(key: SortKey): SortQuery {
        return { key, order: sortQueryOrder() }

        function sortQueryOrder(): SortOrder {
            if (sort.key !== key) {
                return "normal"
            }
            switch (sort.order) {
                case "normal":
                    return "reverse"

                case "reverse":
                    return "normal"
            }
        }
    }

    function sortSign(key: SortKey) {
        if (sort.key !== key) {
            return ""
        }
        return sort.sign[sort.order]
    }
}

export function tableViewColumns(content: VNodeContent): VNode {
    return html`<section class="table__viewColumns">${content}</section>`
}

export type PagerOptionsContent = Readonly<{
    all: number
    step: number
    content: { (params: PagerOptionsContentParams): VNodeContent }
}>
export type PagerOptionsContentParams = Readonly<{ start: number; end: number }>
export function pagerOptions({ all, step, content }: PagerOptionsContent): VNode[] {
    const options: VNode[] = []
    for (let i = 0; i < Math.ceil(all / step); i++) {
        const offset = i * step
        const params = { start: offset + 1, end: end(offset) }
        options.push(html`<option value=${offset}>${content(params)}</option>`)
    }
    return options

    function end(offset: number) {
        if (all < offset + step) {
            return all
        }
        return offset + step
    }
}

type TableType = "normal" | "small" | "fill" | "noMargin"
function tableClass(type: TableType): string {
    switch (type) {
        case "normal":
            return ""

        default:
            return `table_${type}`
    }
}

export function table(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["normal"], sticky, content)
}
export function table_noMargin(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["normal", "noMargin"], sticky, content)
}
export function table_small(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["small"], sticky, content)
}
export function table_small_noMargin(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["small", "noMargin"], sticky, content)
}
export function table_fill(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["fill"], sticky, content)
}
export function table_fill_noMargin(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["fill", "noMargin"], sticky, content)
}
export function table_small_fill(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["small", "fill"], sticky, content)
}
export function table_small_fill_noMargin(sticky: TableDataSticky, content: VNodeContent): VNode {
    return tableContent(["small", "fill", "noMargin"], sticky, content)
}
function tableContent(types: TableType[], sticky: TableDataSticky, content: VNodeContent): VNode {
    return html`<table class="table ${types.map(tableClass).join(" ")} ${stickyTableClass(sticky)}">
        ${content}
    </table>`
}
function stickyTableClass(sticky: TableDataSticky): string {
    switch (sticky.type) {
        case "none":
            return ""

        case "table":
        case "header":
        case "column":
        case "cross":
            return "table_sticky"
    }
}

export function thead(content: VNodeContent): VNode {
    return html`<thead>
        ${content}
    </thead>`
}
export function tbody(content: VNodeContent): VNode {
    return html`<tbody>
        ${content}
    </tbody>`
}
export function tfoot(content: VNodeContent): VNode {
    return html`<tfoot>
        ${content}
    </tfoot>`
}

export type TableHeaderContent =
    | TableHeaderContent_base
    | (TableHeaderContent_base & Readonly<{ singleLastBorderBottom: boolean }>)
type TableHeaderContent_base = Readonly<{
    sticky: TableDataSticky
    header: TableDataHeaderRow
}>
export function tableHeader(content: TableHeaderContent): VNode[] {
    type HeaderRow = Readonly<{
        sticky: StickyHorizontalInfo
        containers: HeaderContainer[]
    }>
    type HeaderContainer = Readonly<{
        index: number
        colspan: number
        rowspan: number
        style: TableDataFullStyle
        header: TableDataHeader
    }>

    type BuildInfo = Readonly<{
        sticky: StickyHorizontalInfo
        index: number
    }>

    const {
        sticky,
        header: { key, className, headers },
    } = content
    const singleLastBorderBottom =
        "singleLastBorderBottom" in content && content.singleLastBorderBottom

    const base: BuildInfo = {
        sticky: { level: 0, borderWidth: 0 },
        index: 0,
    }
    return buildHeaderRows(base, headers).map(headerTr)

    function headerTr({ sticky: info, containers }: HeaderRow): VNode {
        return tr([key(info.level)], className, containers.map(headerTh(info)))
    }

    function headerTh(info: StickyHorizontalInfo): { (container: HeaderContainer): VNode } {
        return ({ index, colspan, rowspan, style, header }) => html`<th
            class="${className(index, style)}"
            colspan=${colspan}
            rowspan=${rowspan}
            key=${header.key}
        >
            ${header.content}
        </th>`

        function className(index: number, style: TableDataFullStyle) {
            return [...styleClass(style), ...stickyHeaderClass(sticky, { info, index })].join(" ")
        }
    }

    function buildHeaderRows(base: BuildInfo, headers: TableDataHeader[]): HeaderRow[] {
        const rowHeight = maxHeight(headers)
        const top = gatherHeader()
        const nextBorderWidth = borderWidth(top)
        return [top, ...gatherChildren()]

        function gatherHeader(): HeaderRow {
            type GatherResult = Readonly<{
                index: number
                containers: HeaderContainer[]
            }>
            return {
                sticky: base.sticky,
                containers: headers.reduce((acc, header) => {
                    return {
                        index: acc.index + header.length,
                        containers: [
                            ...acc.containers,
                            {
                                index: acc.index,
                                colspan: header.length,
                                rowspan: paddingHeight(header) + 1,
                                style: overrideLastBorderBottom(header.style),
                                header,
                            },
                        ],
                    }
                }, initialGatherResult(base.index)).containers,
            }
            function initialGatherResult(index: number): GatherResult {
                return {
                    index,
                    containers: [],
                }
            }
        }

        function gatherChildren(): HeaderRow[] {
            return headers.reduce((acc, header, index) => {
                switch (header.type) {
                    case "simple":
                    case "expansion":
                        return acc

                    case "group":
                        return merge(acc, [
                            ...buildHeaderRows_padding(paddingHeight(header)),
                            ...buildHeaderRows(
                                {
                                    sticky: {
                                        level: base.sticky.level + paddingHeight(header) + 1,
                                        borderWidth: nextBorderWidth,
                                    },
                                    index: base.index + index,
                                },
                                header.children,
                            ),
                        ])
                }
            }, <HeaderRow[]>[])
        }

        function buildHeaderRows_padding(paddingHeight: number): HeaderRow[] {
            return Array(paddingHeight)
                .fill(null)
                .map(
                    (_, i): HeaderRow => {
                        return {
                            sticky: {
                                level: base.sticky.level + 1 + i,
                                borderWidth: nextBorderWidth,
                            },
                            containers: [],
                        }
                    },
                )
        }
        function paddingHeight(header: TableDataHeader): number {
            return rowHeight - header.height
        }

        function merge(base: HeaderRow[], rows: HeaderRow[]): HeaderRow[] {
            return rows.reduce(
                (acc, row, index) => [
                    ...acc.slice(0, index),
                    mergeRow(row, index),
                    ...acc.slice(index + 1),
                ],
                base,
            )

            function mergeRow(row: HeaderRow, index: number): HeaderRow {
                if (index >= base.length) {
                    return row
                }
                const baseRow = base[index]
                return {
                    sticky: {
                        ...baseRow.sticky,
                        borderWidth: Math.max(baseRow.sticky.borderWidth, row.sticky.borderWidth),
                    },
                    containers: [...baseRow.containers, ...row.containers],
                }
            }
        }

        function overrideLastBorderBottom(style: TableDataFullStyle): TableDataFullStyle {
            if (!singleLastBorderBottom) {
                return style
            }
            return overrideBorderBottomToSingle(style)
        }
    }

    function maxHeight(headers: TableDataHeader[]): number {
        return Math.max(0, ...headers.map((header) => header.height))
    }
    function borderWidth(row: HeaderRow): number {
        type BorderInfo = Readonly<{ top: number; bottom: number }>
        return (
            row.sticky.borderWidth +
            sum(
                row.containers.reduce((acc, container) => {
                    return merge({
                        base: acc,
                        border: {
                            top: topWidth(container),
                            bottom: bottomWidth(container),
                        },
                    })
                }, initialBorderInfo()),
            )
        )

        function topWidth(container: HeaderContainer): number {
            return width(container.header.style.border.horizontal.top)
        }
        function bottomWidth(container: HeaderContainer): number {
            if (container.rowspan > 1) {
                return 0
            }
            return width(container.header.style.border.horizontal.bottom)
        }
        function width(border: TableDataBorderClass): number {
            switch (border) {
                case "none":
                case "inherit":
                    return 0

                case "single":
                    return 1 // border-width: 1px

                case "double":
                    return 3 // border-width: 3px
            }
        }

        function initialBorderInfo(): BorderInfo {
            return { top: 0, bottom: 0 }
        }
        function sum({ top, bottom }: BorderInfo): number {
            return top + bottom
        }
        function merge({ base, border }: { base: BorderInfo; border: BorderInfo }): BorderInfo {
            return {
                top: Math.max(base.top, border.top),
                bottom: Math.max(base.bottom, border.bottom),
            }
        }
    }
}

export type TableSummaryContent = Readonly<{
    sticky: TableDataSticky
    summary: TableDataSummaryRow
}>
export function tableSummary({
    sticky,
    summary: { key, className, summaries },
}: TableSummaryContent): VNode[] {
    return [tr([key], className, summaries.map(summaryTd(sticky)))]
}

export type TableColumnContent =
    | TableColumnContent_base
    | (TableColumnContent_base & TableColumnContent_noBorderBottom)
type TableColumnContent_base = Readonly<{
    sticky: TableDataSticky
    column: TableDataColumnRow
}>
type TableColumnContent_noBorderBottom = Readonly<{
    noLastBorderBottom: boolean
}>

export function tableColumn(content: TableColumnContent): VNode[] {
    type ColumnEntry = ColumnEntry_simple | ColumnEntry_expansion | ColumnEntry_tree
    type ColumnEntry_simple = Readonly<{ type: "simple"; container: ColumnContainer }>
    type ColumnEntry_expansion = Readonly<{
        type: "expansion"
        index: number
        column: TableDataColumnExpansion
        containers: ColumnContainer[]
    }>
    type ColumnEntry_tree = Readonly<{
        type: "tree"
        index: number
        column: TableDataColumnTree
        rows: ColumnRow[]
    }>

    type ColumnContainer = Readonly<{
        index: number
        colspan: number
        rowspan: number
        style: TableDataFullStyle
        column: TableDataColumnSimple | EmptyColumn
    }>
    type EmptyColumn = Readonly<{ type: "empty"; key: VNodeKey }>

    type ColumnRow = Readonly<{
        key: VNodeKey[]
        className: TableDataClassName
        containers: ColumnContainer[]
    }>

    type BuildInfo = Readonly<{
        index: number
        bottom: boolean
    }>

    const { sticky, column } = content
    const noLastBorderBottom = "noLastBorderBottom" in content && content.noLastBorderBottom

    return buildColumnRows({ index: 0, bottom: true }, column).map(columnTr)

    function columnTr({ key, className, containers }: ColumnRow): VNode {
        return tr(key, className, containers.map(columnTd))
    }

    function columnTd({ index, colspan, rowspan, style, column }: ColumnContainer): VNode {
        return html`<td
            class="${className()}"
            colspan=${colspan}
            rowspan=${rowspan}
            key=${column.key}
        >
            ${content()}
        </td>`

        function className() {
            return [...styleClass(style), ...stickyColumnClass(sticky, index)].join(" ")
        }
        function content() {
            switch (column.type) {
                case "simple":
                    return column.content

                case "empty":
                    return EMPTY_CONTENT
            }
        }
    }

    function buildColumnRows(base: BuildInfo, source: TableDataColumnRow): ColumnRow[] {
        const rowHeight = maxHeight(source)

        return source.columns
            .reduce(
                (acc, column, index) =>
                    merge(acc, entry(column, { ...base, index: base.index + index })),
                <ColumnRow[]>[],
            )
            .map((row) => {
                return {
                    ...row,
                    key: [source.key, ...row.key],
                    className: [...source.className, ...row.className],
                }
            })

        function entry(column: TableDataColumn, info: BuildInfo): ColumnEntry {
            switch (column.type) {
                case "simple":
                    return simpleEntry(column, info)

                case "expansion":
                    return expansionEntry(column, info)

                case "tree":
                    return treeEntry(column, info)
            }
        }
        function simpleEntry(
            column: TableDataColumnSimple,
            { index }: BuildInfo,
        ): ColumnEntry_simple {
            return {
                type: "simple",
                container: {
                    column,
                    index,
                    colspan: column.length,
                    rowspan: rowHeight,
                    style: overrideLastBorderBottom(column.style),
                },
            }
        }
        function expansionEntry(
            column: TableDataColumnExpansion,
            expansionBase: BuildInfo,
        ): ColumnEntry {
            return {
                type: "expansion",
                index: expansionBase.index,
                column,
                containers: column.columns
                    .slice(0, column.length)
                    .map(
                        (column, index) =>
                            simpleEntry(column, { ...base, index: expansionBase.index + index })
                                .container,
                    ),
            }
        }
        function treeEntry(column: TableDataColumnTree, info: BuildInfo): ColumnEntry {
            return {
                type: "tree",
                index: info.index,
                column,
                rows: column.children.flatMap((row, index) =>
                    buildColumnRows(
                        {
                            ...info,
                            bottom: info.bottom && index === rowHeight - 1,
                        },
                        row,
                    ),
                ),
            }
        }

        function merge(base: ColumnRow[], entry: ColumnEntry): ColumnRow[] {
            switch (entry.type) {
                case "simple":
                    return mergeSimple(entry.container)

                case "expansion":
                    return mergeExpansion(entry)

                case "tree":
                    return mergeTree(entry)
            }

            function mergeSimple(container: ColumnContainer): ColumnRow[] {
                if (base.length === 0) {
                    return [
                        {
                            key: [],
                            className: [],
                            containers: [container],
                        },
                    ]
                }
                return [mergeContainer(base[0]), ...base.slice(1)]

                function mergeContainer(first: ColumnRow): ColumnRow {
                    return { ...first, containers: [...first.containers, container] }
                }
            }

            function mergeExpansion({
                index,
                column,
                containers,
            }: ColumnEntry_expansion): ColumnRow[] {
                if (base.length === 0) {
                    return [
                        {
                            key: [],
                            className: [],
                            containers: expandedContainers(),
                        },
                    ]
                }
                return [mergeContainer(base[0]), ...base.slice(1)]

                function mergeContainer(first: ColumnRow): ColumnRow {
                    return { ...first, containers: [...first.containers, ...expandedContainers()] }
                }

                function expandedContainers(): ColumnContainer[] {
                    return [...containers, ...emptyContainer()]
                }
                function emptyContainer(): ColumnContainer[] {
                    if (containers.length >= column.length) {
                        return []
                    }
                    return [
                        {
                            index,
                            colspan: column.length - containers.length,
                            rowspan: rowHeight,
                            style: overrideLastBorderBottom(column.style),
                            column: { type: "empty", key: `${column.key}__empty` },
                        },
                    ]
                }
            }

            function mergeTree({ index, column, rows }: ColumnEntry_tree): ColumnRow[] {
                return appendEmptyRow(rows).reduce(
                    (acc, row, index) => [
                        ...acc.slice(0, index),
                        mergeRow(row, index),
                        ...acc.slice(index + 1),
                    ],
                    base,
                )

                function mergeRow(row: ColumnRow, index: number): ColumnRow {
                    if (index >= base.length) {
                        return row
                    }
                    const baseRow = base[index]
                    return {
                        key: [...baseRow.key, ...row.key],
                        className: [...baseRow.className, ...row.className],
                        containers: [...baseRow.containers, ...row.containers],
                    }
                }

                function appendEmptyRow(rows: ColumnRow[]): ColumnRow[] {
                    if (rows.length >= rowHeight) {
                        return rows
                    }
                    return [...rows, emptyRow()]

                    function emptyRow(): ColumnRow {
                        return {
                            key: [],
                            className: [],
                            containers: [
                                {
                                    index,
                                    colspan: column.length,
                                    rowspan: rowHeight - rows.length,
                                    style: overrideLastBorderBottom(column.style),
                                    column: {
                                        type: "empty",
                                        key: `__empty_${index}`,
                                    },
                                },
                            ],
                        }
                    }
                }
            }
        }

        function overrideLastBorderBottom(style: TableDataFullStyle): TableDataFullStyle {
            if (!noLastBorderBottom || !base.bottom) {
                return style
            }
            return overrideBorderBottomToNone(style)
        }
    }

    function maxHeight(row: TableDataColumnRow): number {
        return Math.max(1, ...row.columns.map((column) => column.height))
    }
}

export type TableFooterContent = Readonly<{
    sticky: TableDataSticky
    footer: TableDataFooterRow
}>
export function tableFooter({
    sticky,
    footer: { key, className, footers },
}: TableFooterContent): VNode[] {
    return [tr([key], className, footers.map(summaryTd(sticky)))]
}

const summaryTd = (
    sticky: TableDataSticky,
): { (summary: TableDataSummary, index: number): VNode } => (summary, index) => {
    return html`<td class="${className()}" colspan=${summary.length} key=${summary.key}>
        ${summaryContent(summary)}
    </td>`

    function className(): string {
        return [...styleClass(summary.style), ...stickyColumnClass(sticky, index)].join(" ")
    }
}

function summaryContent(summary: TableDataSummary): VNodeContent {
    switch (summary.type) {
        case "empty":
        case "empty-expansion":
            return EMPTY_CONTENT

        case "simple":
        case "expansion":
            return summary.content
    }
}

function tr(key: VNodeKey[], className: TableDataClassName, content: VNodeContent): VNode {
    return html`<tr class="${className.join(" ")}" key=${key.join("_")} data-root-key=${key[0]}>
        ${content}
    </tr>`
}

function styleClass(style: TableDataFullStyle): string[] {
    return [...borderClass(style.border), ...alignClass(style.align), ...style.className]

    function borderClass(border: TableDataBorderStyle): string[] {
        type TypedBorder =
            | Readonly<{ type: "t"; border: TableDataBorderClass }>
            | Readonly<{ type: "b"; border: TableDataBorderClass }>
            | Readonly<{ type: "l"; border: TableDataBorderClass }>
            | Readonly<{ type: "r"; border: TableDataBorderClass }>

        const borders: TypedBorder[] = [
            { type: "t", border: border.horizontal.top },
            { type: "b", border: border.horizontal.bottom },
            { type: "l", border: border.vertical.left },
            { type: "r", border: border.vertical.right },
        ]
        return borders.flatMap(({ type, border }: TypedBorder) => {
            switch (border) {
                case "none":
                case "inherit":
                    return []

                case "single":
                    return `cell_border_${type}`
                case "double":
                    return `cell_border_${type.repeat(2)}`
            }
        })
    }
    function alignClass(align: TableDataAlignStyle): string[] {
        return [align.horizontal, align.vertical].flatMap((type: TableDataAlign) => {
            switch (type) {
                case "inherit":
                    return []

                case "left":
                case "center":
                case "right":
                case "numeric":
                case "top":
                case "middle":
                case "baseline":
                case "bottom":
                    return `cell_${type}`
            }
        })
    }
}

type StickyHorizontalInfo = Readonly<{
    level: number
    borderWidth: number
}>
type StickyHeaderContent = Readonly<{
    info: StickyHorizontalInfo
    index: number
}>
function stickyHeaderClass(
    sticky: TableDataSticky,
    { info: { level, borderWidth }, index }: StickyHeaderContent,
): string[] {
    switch (sticky.type) {
        case "none":
        case "table":
        case "column":
            return []

        case "header":
            return stickyHeader()

        case "cross":
            if (!isStickyColumn(sticky, index)) {
                return stickyHeader()
            }
            return [
                "cell_sticky",
                "cell_sticky_cross",
                stickyTopClass({ level, borderWidth: borderWidth }),
                stickyLeftClass(index),
            ]
    }

    function stickyHeader() {
        return ["cell_sticky", stickyTopClass({ level, borderWidth })]
    }
}
function stickyColumnClass(sticky: TableDataSticky, index: number): string[] {
    if (!isStickyColumn(sticky, index)) {
        return []
    }
    return ["cell_sticky", stickyLeftClass(index)]
}
function isStickyColumn(sticky: TableDataSticky, index: number): boolean {
    switch (sticky.type) {
        case "none":
        case "table":
        case "header":
            return false

        case "column":
        case "cross":
            return index < sticky.column
    }
}
type StickyTopContent = Readonly<{ level: number; borderWidth: number }>
function stickyTopClass({ level, borderWidth }: StickyTopContent) {
    return `cell_sticky_top${indexToClass(level)}${borderWidthToClass(borderWidth)}`
}
function stickyLeftClass(index: number) {
    return `cell_sticky_left${indexToClass(index)}`
}
function indexToClass(index: number): string {
    if (index === 0) {
        return ""
    }
    return (index + 1).toString()
}
function borderWidthToClass(borderWidth: number): string {
    if (borderWidth === 0) {
        return ""
    }
    return `_${borderWidth}`
}

const EMPTY_CONTENT = html``
