import {
    TableDataColumn,
    TableDataHeader,
    TableDataParams,
    TableDataSummary,
    TableDataView,
} from "../core"

import { TableDataMutable_base } from "../mutable"
import { tableDataMutable_base } from "../mutable/base"
import {
    tableCellColumn,
    tableCellFooter,
    tableCellHeader,
    tableCellSummary,
    tableCellView,
    TableCell,
    TableCellMultipart,
    TableDataMultipartProvider,
    TableDataRelatedParams,
    TableDataStyledParams,
} from "../cell"
import {
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataSummaryDecorator,
} from "../decorator"
import { TableDataHorizontalBorder } from "../style"

export type TableDataMultipartContent<M, R, P> = Readonly<{
    data: TableDataMultipartProvider<M, P>
    cells: TableDataMultipartCellProvider<M, R, P>
}>
export function tableCell_multipart<M, R, P>(
    content: TableDataMultipartContent<M, R, P>
): TableCellMultipart<M, R> {
    return new Cell(content)
}
class Cell<M, R, P> implements TableCellMultipart<M, R> {
    readonly type = "multipart" as const

    content: TableDataMultipartContent<M, R, P>
    mutable: Readonly<{
        core: TableDataMutable_base<R>
    }>

    constructor(content: TableDataMultipartContent<M, R, P>) {
        this.content = content
        this.mutable = {
            core: tableDataMutable_base(),
        }
    }

    cells(model: M): TableCell<M, R>[] {
        return this.content.data(model).flatMap((part) => this.content.cells(part))
    }

    view(params: TableDataParams<M>): TableDataView[] {
        return tableCellView(params, this.cells(params.model))
    }
    header(params: TableDataStyledParams<M>): TableDataHeader[] {
        const { style } = this.mutable.core.headerStyleMutable()
        return tableCellHeader(params, style, this.cells(params.model))
    }
    summary(params: TableDataStyledParams<M>): TableDataSummary[] {
        const { style } = this.mutable.core.summaryStyleMutable()
        return tableCellSummary(params, style, this.cells(params.model))
    }
    column(params: TableDataRelatedParams<M, R>): TableDataColumn[] {
        const { style } = this.mutable.core.columnStyleMutable()
        const { decorators } = this.mutable.core.columnMutable()
        return tableCellColumn(params, style, decorators, this.cells(params.model))
    }
    footer(params: TableDataStyledParams<M>): TableDataSummary[] {
        const { style } = this.mutable.core.footerStyleMutable()
        return tableCellFooter(params, style, this.cells(params.model))
    }

    horizontalBorder(borders: TableDataHorizontalBorder[]): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorder(borders)
        return this
    }
    horizontalBorderRelated(borders: TableDataHorizontalBorderProvider<R>): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorderRelated(borders)
        return this
    }
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorder_header(borders)
        return this
    }
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorder_summary(borders)
        return this
    }
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorder_footer(borders)
        return this
    }

    decorateHeader(decorator: TableDataHeaderDecorator): TableCellMultipart<M, R> {
        this.mutable.core.decorateHeader(decorator)
        return this
    }
    decorateSummary(decorator: TableDataSummaryDecorator): TableCellMultipart<M, R> {
        this.mutable.core.decorateSummary(decorator)
        return this
    }
    decorateColumn(decorator: TableDataColumnDecorator): TableCellMultipart<M, R> {
        this.mutable.core.decorateColumn(decorator)
        return this
    }
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): TableCellMultipart<M, R> {
        this.mutable.core.decorateColumnRelated(decorator)
        return this
    }
    decorateFooter(decorator: TableDataSummaryDecorator): TableCellMultipart<M, R> {
        this.mutable.core.decorateFooter(decorator)
        return this
    }
}

interface TableDataMultipartCellProvider<M, R, C> {
    (child: C): TableCell<M, R>[]
}
