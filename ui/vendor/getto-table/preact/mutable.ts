import { TableDataHeaderKeyProvider, TableDataKeyProvider } from "./core"
import {
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataContentDecoratorProvider,
    TableDataGroupDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataRowDecorator,
    TableDataRowRelatedDecorator,
    TableDataSummaryDecorator,
    TableDataViewDecorator,
} from "./decorator"
import {
    TableDataHorizontalBorder,
    TableDataRowStyle,
    TableDataSticky,
    TableDataStyle,
    TableDataVerticalBorder,
    TableDataVerticalBorderStyle,
    TableDataVisibleType,
} from "./style"

export interface TableDataMutable_base<R> {
    headerStyleMutable(): TableDataStyleMutable
    summaryStyleMutable(): TableDataStyleMutable
    columnStyleMutable(): TableDataStyleMutable
    footerStyleMutable(): TableDataStyleMutable

    columnMutable(): TableDataColumnMutable<R>

    horizontalBorder(borders: TableDataHorizontalBorder[]): void
    horizontalBorderRelated(borders: TableDataHorizontalBorderProvider<R>): void
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): void
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): void
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): void

    decorateHeader(decorator: TableDataHeaderDecorator): void
    decorateSummary(decorator: TableDataSummaryDecorator): void
    decorateColumn(decorator: TableDataColumnDecorator): void
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): void
    decorateFooter(decorator: TableDataSummaryDecorator): void
}
export interface TableDataMutable_leaf {
    visibleMutable(): TableDataVisibleMutable
    viewMutable(): TableDataViewMutable
    verticalBorderMutable(): TableDataVerticalBorderMutable

    alwaysVisible(): void
    border(borders: TableDataVerticalBorder[]): void

    decorateView(decorator: TableDataViewDecorator): void
}
export interface TableDataMutable_group {
    viewMutable(): TableDataViewMutable
    groupStyleMutable(): TableDataStyleMutable

    horizontalBorder_group(borders: TableDataHorizontalBorder[]): void

    decorateView(decorator: TableDataViewDecorator): void
    decorateGroup(decorator: TableDataGroupDecorator): void
}
export interface TableDataMutable_tree<R> {
    rowMutable(): TableDataRowMutable<R>

    decorateRow(decorator: TableDataRowDecorator): void
    decorateRowRelated(decorator: TableDataRowRelatedDecorator<R>): void
}
export interface TableDataMutable_row {
    headerRowMutable(): TableDataHeaderRowMutable
    summaryRowMutable(): TableDataSummaryRowMutable
    footerRowMutable(): TableDataSummaryRowMutable
    stickyMutable(): TableDataStickyMutable

    setHeaderKey(key: TableDataHeaderKeyProvider): void
    setSummaryKey(key: TableDataKeyProvider): void
    setFooterKey(key: TableDataKeyProvider): void

    decorateHeaderRow(decorator: TableDataRowDecorator): void
    decorateSummaryRow(decorator: TableDataRowDecorator): void
    decorateFooterRow(decorator: TableDataRowDecorator): void

    stickyTable(): void
    stickyHeader(): void
    stickyColumn(n: number): void
    stickyCross(n: number): void
}

export type TableDataStyleMutable = Readonly<{
    style: TableDataStyle
}>
export type TableDataVisibleMutable = Readonly<{
    visibleType: TableDataVisibleType
}>
export type TableDataViewMutable = Readonly<{
    decorator: TableDataContentDecoratorProvider
}>
export type TableDataColumnMutable<R> = Readonly<{
    decorators: TableDataColumnRelatedDecorator<R>[]
}>
export type TableDataRowMutable<R> = Readonly<{
    style: TableDataRowStyle
    decorators: TableDataRowRelatedDecorator<R>[]
}>
export type TableDataHeaderRowMutable = Readonly<{
    key: TableDataHeaderKeyProvider
    style: TableDataRowStyle
}>
export type TableDataSummaryRowMutable = Readonly<{
    key: TableDataKeyProvider
    style: TableDataRowStyle
}>
export type TableDataStickyMutable = Readonly<{
    sticky: TableDataSticky
}>

export type TableDataVerticalBorderMutable = Readonly<{
    border: TableDataVerticalBorderStyle
}>
