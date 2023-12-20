import { PreactContent, PreactKey } from "./common"

import { TableDataClassName, TableDataFullStyle, TableDataSticky } from "./style"

export interface TableStructure<M, R> {
    sticky(): TableDataSticky
    allCells(): readonly TableDataCell[]
    header(params: TableDataParams<M>): TableDataHeaderRow
    summary(params: TableDataParams<M>): TableDataSummaryRow
    column(params: TableDataParams<M>, row: R): TableDataColumnRow
    footer(params: TableDataParams<M>): TableDataFooterRow
}

export type TableDataKey = string
export type TableDataParams<M> = Readonly<{ summary: M }> &
    Partial<{ visibleKeys: readonly TableDataKey[] }>

export type TableDataCell = Readonly<{
    key: TableDataKey
    content: PreactContent
    isInitiallyVisible: boolean
}>

export type TableDataHeader =
    | TableDataHeaderSimple
    | TableDataHeaderExpansion
    | TableDataHeaderGroup

export type TableDataHeaderSimple = Readonly<{
    type: "simple"
    key: PreactKey
    style: TableDataFullStyle
    content: PreactContent
    height: 1
    length: 1
}>
export type TableDataHeaderExpansion = Readonly<{
    type: "expansion"
    key: PreactKey
    style: TableDataFullStyle
    content: PreactContent
    height: 1
    length: number
}>
export type TableDataHeaderGroup = Readonly<{
    type: "group"
    key: PreactKey
    style: TableDataFullStyle
    content: PreactContent
    children: readonly TableDataHeader[]
    height: number
    length: number
}>

export type TableDataSummary = TableDataSummarySimple | TableDataSummaryExpansion

export type TableDataSummarySimple =
    | (TableDataSummarySimple_base & Readonly<{ type: "empty" }>)
    | (TableDataSummarySimple_base & Readonly<{ type: "simple"; content: PreactContent }>)
type TableDataSummarySimple_base = Readonly<{
    key: PreactKey
    style: TableDataFullStyle
    length: 1
}>

export type TableDataSummaryExpansion =
    | (TableDataSummaryExpansion_base & Readonly<{ type: "empty-expansion" }>)
    | (TableDataSummaryExpansion_base & Readonly<{ type: "expansion"; content: PreactContent }>)
type TableDataSummaryExpansion_base = Readonly<{
    key: PreactKey
    style: TableDataFullStyle
    length: number
}>

export type TableDataColumn = TableDataColumnSimple | TableDataColumnExpansion | TableDataColumnTree

export type TableDataColumnSimple = Readonly<{
    type: "simple"
    key: PreactKey
    style: TableDataFullStyle
    content: PreactContent
    length: 1
    height: 1
}>
export type TableDataColumnExpansion = Readonly<{
    type: "expansion"
    key: PreactKey
    style: TableDataFullStyle
    length: number
    height: 1
    columns: readonly TableDataColumnSimple[]
}>
export type TableDataColumnTree = Readonly<{
    type: "tree"
    children: readonly TableDataColumnRow[]
    length: number
    height: number
    style: TableDataFullStyle
}>

export type TableCellTreePaddingContent = Readonly<{
    key: PreactKey
    rowHeight: number
    column: TableDataColumnTree
}>

export type TableDataHeaderRow = Readonly<{
    key: TableDataHeaderKeyProvider
    className: TableDataClassName
    headers: readonly TableDataHeader[]
}>
export type TableDataSummaryRow = Readonly<{
    key: PreactKey
    className: TableDataClassName
    summaries: readonly TableDataSummary[]
}>
export type TableDataColumnRow = Readonly<{
    key: PreactKey
    className: TableDataClassName
    columns: readonly TableDataColumn[]
}>
export type TableDataFooterRow = Readonly<{
    key: PreactKey
    className: TableDataClassName
    footers: readonly TableDataSummary[]
}>

export interface TableDataHeaderKeyProvider {
    (index: number): PreactKey
}
export interface TableDataKeyProvider {
    (): PreactKey
}
