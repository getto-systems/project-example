import { TableDataMutable_base, TableDataMutable_row, TableDataMutable_tree } from "../mutable"
import { tableDataMutable_default } from "../mutable/base"
import { tableDataMutable_tree } from "../mutable/tree"
import {
    tableCellView,
    tableCellBaseHeader,
    tableCellBaseSummary,
    tableCellBaseColumn,
    tableCellBaseFooter,
    TableCell,
    TableDataRowKeyProvider,
    TableStructure_hot,
} from "../cell"
import {
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataRowDecorator,
    TableDataRowRelatedDecorator,
    TableDataSummaryDecorator,
} from "../decorator"
import { TableDataHorizontalBorder, TableDataSticky } from "../style"
import {
    TableDataColumnRow,
    TableDataFooterRow,
    TableDataHeaderKeyProvider,
    TableDataHeaderRow,
    TableDataKeyProvider,
    TableDataParams,
    TableDataSummaryRow,
    TableDataView,
    TableStructure,
} from "../core"
import { tableDataMutable_row } from "../mutable/row"

export function tableStructure<M, R>(
    key: TableDataRowKeyProvider<R>,
    cells: TableCell<M, R>[],
): TableStructure_hot<M, R> {
    return new Structure(key, cells)
}
class Structure<M, R> implements TableStructure<M, R>, TableStructure_hot<M, R> {
    key: TableDataRowKeyProvider<R>
    cells: TableCell<M, R>[]
    mutable: Readonly<{
        core: TableDataMutable_base<R>
        tree: TableDataMutable_tree<R>
        row: TableDataMutable_row
    }>

    constructor(key: TableDataRowKeyProvider<R>, cells: TableCell<M, R>[]) {
        this.key = key
        this.cells = cells
        this.mutable = {
            core: tableDataMutable_default(),
            tree: tableDataMutable_tree(),
            row: tableDataMutable_row(),
        }
    }

    view(params: TableDataParams<M>): TableDataView[] {
        return tableCellView(params, this.cells)
    }
    header(params: TableDataParams<M>): TableDataHeaderRow {
        const { style } = this.mutable.core.headerStyleMutable()
        const headerRow = this.mutable.row.headerRowMutable()
        return {
            key: headerRow.key,
            className: headerRow.style.className,
            headers: tableCellBaseHeader(params, style, this.cells),
        }
    }
    summary(params: TableDataParams<M>): TableDataSummaryRow {
        const { style } = this.mutable.core.summaryStyleMutable()
        const summaryRow = this.mutable.row.summaryRowMutable()
        return {
            key: summaryRow.key(),
            className: summaryRow.style.className,
            summaries: tableCellBaseSummary(params, style, this.cells),
        }
    }
    column(params: TableDataParams<M>, row: R): TableDataColumnRow {
        const { style } = this.mutable.core.columnStyleMutable()
        const treeRow = this.mutable.tree.rowMutable()
        const { decorators } = this.mutable.core.columnMutable()
        return {
            key: this.key(row),
            className: treeRow.style.className,
            columns: tableCellBaseColumn(params, style, decorators, this.cells, row),
        }
    }
    footer(params: TableDataParams<M>): TableDataFooterRow {
        const { style } = this.mutable.core.footerStyleMutable()
        const footerRow = this.mutable.row.footerRowMutable()
        return {
            key: footerRow.key(),
            className: footerRow.style.className,
            footers: tableCellBaseFooter(params, style, this.cells),
        }
    }

    horizontalBorder(borders: TableDataHorizontalBorder[]): TableStructure_hot<M, R> {
        this.mutable.core.horizontalBorder(borders)
        return this
    }
    horizontalBorderRelated(
        borders: TableDataHorizontalBorderProvider<R>,
    ): TableStructure_hot<M, R> {
        this.mutable.core.horizontalBorderRelated(borders)
        return this
    }
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): TableStructure_hot<M, R> {
        this.mutable.core.horizontalBorder_header(borders)
        return this
    }
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): TableStructure_hot<M, R> {
        this.mutable.core.horizontalBorder_summary(borders)
        return this
    }
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): TableStructure_hot<M, R> {
        this.mutable.core.horizontalBorder_footer(borders)
        return this
    }

    decorateHeader(decorator: TableDataHeaderDecorator): TableStructure_hot<M, R> {
        this.mutable.core.decorateHeader(decorator)
        return this
    }
    decorateSummary(decorator: TableDataSummaryDecorator): TableStructure_hot<M, R> {
        this.mutable.core.decorateSummary(decorator)
        return this
    }
    decorateColumn(decorator: TableDataColumnDecorator): TableStructure_hot<M, R> {
        this.mutable.core.decorateColumn(decorator)
        return this
    }
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): TableStructure_hot<M, R> {
        this.mutable.core.decorateColumnRelated(decorator)
        return this
    }
    decorateFooter(decorator: TableDataSummaryDecorator): TableStructure_hot<M, R> {
        this.mutable.core.decorateFooter(decorator)
        return this
    }
    decorateHeaderRow(decorator: TableDataRowDecorator): TableStructure_hot<M, R> {
        this.mutable.row.decorateHeaderRow(decorator)
        return this
    }
    decorateSummaryRow(decorator: TableDataRowDecorator): TableStructure_hot<M, R> {
        this.mutable.row.decorateSummaryRow(decorator)
        return this
    }
    decorateFooterRow(decorator: TableDataRowDecorator): TableStructure_hot<M, R> {
        this.mutable.row.decorateFooterRow(decorator)
        return this
    }
    decorateRow(decorator: TableDataRowDecorator): TableStructure_hot<M, R> {
        this.mutable.tree.decorateRow(decorator)
        return this
    }
    decorateRowRelated(decorator: TableDataRowRelatedDecorator<R>): TableStructure_hot<M, R> {
        this.mutable.tree.decorateRowRelated(decorator)
        return this
    }

    setHeaderKey(key: TableDataHeaderKeyProvider): TableStructure_hot<M, R> {
        this.mutable.row.setHeaderKey(key)
        return this
    }
    setSummaryKey(key: TableDataKeyProvider): TableStructure_hot<M, R> {
        this.mutable.row.setSummaryKey(key)
        return this
    }
    setFooterKey(key: TableDataKeyProvider): TableStructure_hot<M, R> {
        this.mutable.row.setFooterKey(key)
        return this
    }

    stickyTable(): TableStructure_hot<M, R> {
        this.mutable.row.stickyTable()
        return this
    }
    stickyHeader(): TableStructure_hot<M, R> {
        this.mutable.row.stickyHeader()
        return this
    }
    stickyColumn(column: number): TableStructure_hot<M, R> {
        this.mutable.row.stickyColumn(column)
        return this
    }
    stickyCross(column: number): TableStructure_hot<M, R> {
        this.mutable.row.stickyCross(column)
        return this
    }

    freeze(): TableStructure<M, R> {
        return this
    }
    sticky(): TableDataSticky {
        const { sticky } = this.mutable.row.stickyMutable()
        return sticky
    }
}
