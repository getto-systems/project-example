import {
    TableDataKey,
    TableDataColumn,
    TableDataHeader,
    TableDataSummary,
    TableDataCell,
} from "../core"

import { TableDataMutable_base } from "../mutable"
import { tableDataMutable_base } from "../mutable/base"
import {
    tableCellColumn,
    tableCellFooter,
    tableCellHeader,
    tableCellSummary,
    TableCell,
    TableCellMultipart,
    TableDataMultipartProvider,
    TableDataRelatedParams,
    TableDataStyledParams,
    TableDataInherit,
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
    content: TableDataMultipartContent<M, R, P>,
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

    cells(summary: M): readonly TableCell<M, R>[] {
        return this.content.data(summary).flatMap((part) => this.content.cells(part))
    }

    initiallyVisibleCells(): readonly TableDataKey[] {
        // multipart の cell はデータを取得しないといけない
        // データを取得してから初期表示セルを判定していたのでは遅いので、
        // multipart の cell は always visible 扱いとする
        return []
    }

    view(): readonly TableDataCell[] {
        // multipart の cell はデータを取得しないといけない
        // データを取得してから初期表示セルを判定していたのでは遅いので、
        // multipart の cell は always visible 扱いとする
        return []
    }
    header(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): readonly TableDataHeader[] {
        const { style } = this.mutable.core.headerStyleMutable()
        return tableCellHeader(
            { ...inherit, isInMultipart: true },
            params,
            style,
            this.cells(params.summary),
        )
    }
    summary(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): readonly TableDataSummary[] {
        const { style } = this.mutable.core.summaryStyleMutable()
        return tableCellSummary(
            { ...inherit, isInMultipart: true },
            params,
            style,
            this.cells(params.summary),
        )
    }
    column(
        inherit: TableDataInherit,
        params: TableDataRelatedParams<M, R>,
    ): readonly TableDataColumn[] {
        const { style } = this.mutable.core.columnStyleMutable()
        const { decorators } = this.mutable.core.columnMutable()
        return tableCellColumn(
            { ...inherit, isInMultipart: true },
            params,
            style,
            decorators,
            this.cells(params.summary),
        )
    }
    footer(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): readonly TableDataSummary[] {
        const { style } = this.mutable.core.footerStyleMutable()
        return tableCellFooter(
            { ...inherit, isInMultipart: true },
            params,
            style,
            this.cells(params.summary),
        )
    }

    horizontalBorder(borders: readonly TableDataHorizontalBorder[]): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorder(borders)
        return this
    }
    horizontalBorderRelated(
        borders: TableDataHorizontalBorderProvider<R>,
    ): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorderRelated(borders)
        return this
    }
    horizontalBorder_header(
        borders: readonly TableDataHorizontalBorder[],
    ): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorder_header(borders)
        return this
    }
    horizontalBorder_summary(
        borders: readonly TableDataHorizontalBorder[],
    ): TableCellMultipart<M, R> {
        this.mutable.core.horizontalBorder_summary(borders)
        return this
    }
    horizontalBorder_footer(
        borders: readonly TableDataHorizontalBorder[],
    ): TableCellMultipart<M, R> {
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
    (child: C): readonly TableCell<M, R>[]
}
