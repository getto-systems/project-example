import { VNodeContent, VNodeKey } from "./common"

import {
    TableDataColumn,
    TableDataColumnExpansion,
    TableDataColumnSimple,
    TableDataColumnTree,
    TableDataHeader,
    TableDataHeaderExpansion,
    TableDataHeaderGroup,
    TableDataHeaderKeyProvider,
    TableDataHeaderSimple,
    TableDataSummary,
    TableDataSummaryExpansion,
    TableDataKeyProvider,
    TableDataSummarySimple,
    TableDataView,
    TableStructure,
    TableDataParams,
    TableDataCellKey,
} from "./core"

import {
    decorateStyle,
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataGroupDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataRowDecorator,
    TableDataRowRelatedDecorator,
    TableDataSummaryDecorator,
    TableDataViewDecorator,
} from "./decorator"
import {
    extendStyle,
    overrideBorderBottom,
    TableDataHorizontalBorder,
    TableDataStyle,
    TableDataVerticalBorder,
} from "./style"

export type TableCell<M, R> =
    | TableCellSimple<M, R>
    | TableCellExpansion<M, R>
    | TableCellGroup<M, R>
    | TableCellMultipart<M, R>
    | TableCellTree<M, R>

export interface TableCellSimple<M, R>
    extends TableCell_base<TableCellSimple<M, R>, R>,
        TableCell_leaf<TableCellSimple<M, R>> {
    type: "simple"

    initiallyVisibleCells(): TableDataCellKey[]
    view(): TableDataView[]
    header(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataHeaderSimple | TableDataInvisible
    summary(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummarySimple | TableDataInvisible
    column(
        inherit: TableDataInherit,
        params: TableDataRelatedParams<M, R>,
    ): TableDataColumnSimple | TableDataInvisible
    footer(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummarySimple | TableDataInvisible
}
export interface TableCellExpansion<M, R>
    extends TableCell_base<TableCellExpansion<M, R>, R>,
        TableCell_leaf<TableCellExpansion<M, R>> {
    type: "expansion"

    initiallyVisibleCells(): TableDataCellKey[]
    view(): TableDataView[]
    header(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataHeaderExpansion | TableDataInvisible
    summary(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummaryExpansion | TableDataInvisible
    column(
        inherit: TableDataInherit,
        params: TableDataRelatedParams<M, R>,
    ): TableDataColumnExpansion | TableDataInvisible
    footer(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummaryExpansion | TableDataInvisible
}
export interface TableCellGroup<M, R>
    extends TableCell_base<TableCellGroup<M, R>, R>,
        TableCell_group<TableCellGroup<M, R>> {
    type: "group"

    initiallyVisibleCells(): TableDataCellKey[]
    view(): TableDataView[]
    header(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataHeaderGroup | TableDataInvisible
    summary(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataSummary[]
    column(inherit: TableDataInherit, params: TableDataRelatedParams<M, R>): TableDataColumn[]
    footer(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataSummary[]
}
export interface TableCellMultipart<M, R> extends TableCell_base<TableCellMultipart<M, R>, R> {
    type: "multipart"

    initiallyVisibleCells(): TableDataCellKey[]
    view(): TableDataView[]
    header(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataHeader[]
    summary(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataSummary[]
    column(inherit: TableDataInherit, params: TableDataRelatedParams<M, R>): TableDataColumn[]
    footer(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataSummary[]
}
export interface TableCellTree<M, R>
    extends TableCell_base<TableCellTree<M, R>, R>,
        TableCell_tree<TableCellTree<M, R>, R> {
    type: "tree"

    initiallyVisibleCells(): TableDataCellKey[]
    view(): TableDataView[]
    header(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataHeader[]
    summary(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataSummary[]
    column(inherit: TableDataInherit, params: TableDataRelatedParams<M, R>): TableDataColumnTree
    footer(inherit: TableDataInherit, params: TableDataStyledParams<M>): TableDataSummary[]
}

interface TableCell_base<T, R> {
    horizontalBorder(borders: TableDataHorizontalBorder[]): T
    horizontalBorderRelated(borders: TableDataHorizontalBorderProvider<R>): T
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): T
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): T
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): T

    decorateHeader(decorator: TableDataHeaderDecorator): T
    decorateSummary(decorator: TableDataSummaryDecorator): T
    decorateColumn(decorator: TableDataColumnDecorator): T
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): T
    decorateFooter(decorator: TableDataSummaryDecorator): T
}
interface TableCell_leaf<T> {
    alwaysVisible(): T
    border(borders: TableDataVerticalBorder[]): T

    decorateView(decorator: TableDataViewDecorator): T
}
interface TableCell_group<T> {
    horizontalBorder_group(borders: TableDataHorizontalBorder[]): T

    decorateView(decorator: TableDataViewDecorator): T
    decorateGroup(decorator: TableDataGroupDecorator): T
}
interface TableCell_tree<T, R> {
    decorateRow(decorator: TableDataRowDecorator): T
    decorateRowRelated(decorator: TableDataRowRelatedDecorator<R>): T
}
interface TableCell_structure<T> {
    setHeaderKey(key: TableDataHeaderKeyProvider): T
    setSummaryKey(key: TableDataKeyProvider): T
    setFooterKey(key: TableDataKeyProvider): T

    decorateHeaderRow(decorator: TableDataRowDecorator): T
    decorateSummaryRow(decorator: TableDataRowDecorator): T
    decorateFooterRow(decorator: TableDataRowDecorator): T

    stickyTable(): T
    stickyHeader(): T
    stickyColumn(n: number): T
    stickyCross(n: number): T
}

export interface TableStructure_hot<M, R>
    extends TableCell_base<TableStructure_hot<M, R>, R>,
        TableCell_tree<TableStructure_hot<M, R>, R>,
        TableCell_structure<TableStructure_hot<M, R>> {
    freeze(): TableStructure<M, R>
}

export type TableDataInherit = Readonly<{
    isInMultipart: boolean
}>

export type TableDataStyledParams<M> = TableDataParams<M> & Readonly<{ base: TableDataStyle }>
export type TableDataRelatedParams<M, R> = TableDataStyledParams<M> & Readonly<{ row: R }>

export type TableDataInvisible = Readonly<{ type: "invisible" }>

export interface TableDataColumnContentProvider<R> {
    (row: R): VNodeContent
}
export interface TableDataExpansionColumnContentProvider<R> {
    (row: R): VNodeContent[]
}

export interface TableDataRowKeyProvider<R> {
    (row: R): VNodeKey
}
export interface TableDataMultipartProvider<M, P> {
    (model: M): P[]
}
export interface TableDataTreeChildrenProvider<M, R, C> {
    (row: R, model: M): C[]
}

export function tableCellInitiallyVisibleCells<M, R>(cells: TableCell<M, R>[]): TableDataCellKey[] {
    return cells.flatMap((cell) => cell.initiallyVisibleCells())
}
export function tableCellView<M, R>(cells: TableCell<M, R>[]): TableDataView[] {
    return cells.flatMap((cell) => cell.view())
}
export function tableCellHeader<M, R>(
    inherit: TableDataInherit,
    params: TableDataStyledParams<M>,
    style: TableDataStyle,
    cells: TableCell<M, R>[],
): TableDataHeader[] {
    return tableCellBaseHeader(inherit, params, extendStyle({ base: params.base, style }), cells)
}
export function tableCellBaseHeader<M, R>(
    inherit: TableDataInherit,
    params: TableDataParams<M>,
    base: TableDataStyle,
    cells: TableCell<M, R>[],
): TableDataHeader[] {
    return withoutInvisible(cells.flatMap((cell) => cell.header(inherit, { ...params, base })))
}
export function tableCellSummary<M, R>(
    inherit: TableDataInherit,
    params: TableDataStyledParams<M>,
    style: TableDataStyle,
    cells: TableCell<M, R>[],
): TableDataSummary[] {
    return tableCellBaseSummary(inherit, params, extendStyle({ base: params.base, style }), cells)
}
export function tableCellBaseSummary<M, R>(
    inherit: TableDataInherit,
    params: TableDataParams<M>,
    base: TableDataStyle,
    cells: TableCell<M, R>[],
): TableDataSummary[] {
    return withoutInvisible(cells.flatMap((cell) => cell.summary(inherit, { ...params, base })))
}
export function tableCellColumn<M, R>(
    inherit: TableDataInherit,
    params: TableDataRelatedParams<M, R>,
    style: TableDataStyle,
    decorators: TableDataColumnRelatedDecorator<R>[],
    cells: TableCell<M, R>[],
): TableDataColumn[] {
    // decorate してから extend したいから Base は使えない
    return withoutInvisible(
        cells.flatMap((cell) =>
            cell.column(inherit, {
                ...params,
                base: extendStyle({ base: params.base, style: decorated(style) }),
            }),
        ),
    )

    function decorated(style: TableDataStyle) {
        return decorators.reduce(
            (acc, decorator) => decorateStyle(acc, decorator(params.row)),
            style,
        )
    }
}
export function tableCellBaseColumn<M, R>(
    inherit: TableDataInherit,
    params: TableDataParams<M>,
    base: TableDataStyle,
    decorators: TableDataColumnRelatedDecorator<R>[],
    cells: TableCell<M, R>[],
    row: R,
): TableDataColumn[] {
    return withoutInvisible(
        cells.flatMap((cell) => cell.column(inherit, { ...params, base: decorated(base), row })),
    )

    function decorated(style: TableDataStyle) {
        return decorators.reduce((acc, decorator) => decorateStyle(acc, decorator(row)), style)
    }
}
export function tableCellChildColumn<M, R, C>(
    inherit: TableDataInherit,
    params: TableDataRelatedParams<M, R>,
    base: TableDataStyle,
    decorators: TableDataColumnRelatedDecorator<R>[],
    cells: TableCell<M, C>[],
    child: Readonly<{ row: C; last: boolean }>,
): TableDataColumn[] {
    // decorate してから extend したいから Base は使えない
    return withoutInvisible(
        cells.flatMap((cell) =>
            cell.column(inherit, {
                ...params,
                row: child.row,
                base: extendStyle({ base: params.base, style: decorated(base) }),
            }),
        ),
    ).map(overrideLastChildBorderBottom)

    function decorated(style: TableDataStyle) {
        return decorators.reduce(
            (acc, decorator) => decorateStyle(acc, decorator(params.row)),
            style,
        )
    }

    function overrideLastChildBorderBottom(column: TableDataColumn): TableDataColumn {
        if (!child.last) {
            return column
        }
        return {
            ...column,
            style: overrideBorderBottom(column.style, params.base.horizontalBorder),
        }
    }
}
export function tableCellFooter<M, R>(
    inherit: TableDataInherit,
    params: TableDataStyledParams<M>,
    style: TableDataStyle,
    cells: TableCell<M, R>[],
): TableDataSummary[] {
    return tableCellBaseFooter(inherit, params, extendStyle({ base: params.base, style }), cells)
}
export function tableCellBaseFooter<M, R>(
    inherit: TableDataInherit,
    params: TableDataParams<M>,
    base: TableDataStyle,
    cells: TableCell<M, R>[],
): TableDataSummary[] {
    return withoutInvisible(cells.flatMap((cell) => cell.footer(inherit, { ...params, base })))
}

function withoutInvisible<T>(entries: (TableDataInvisible | TableDataTypedContent<T>)[]): T[] {
    // T と invisible の中から invisible を取り除くと T[] になる
    return entries.filter((entry) => entry.type !== "invisible") as T[]
}
type TableDataTypedContent<T> = T & Readonly<{ type: string }>
