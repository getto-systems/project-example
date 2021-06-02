import {
    TableDataColumnRow,
    TableDataColumnTree,
    TableDataHeader,
    TableDataParams,
    TableDataSummary,
    TableDataView,
} from "../core"

import { TableDataMutable_base, TableDataMutable_tree } from "../mutable"
import { tableDataMutable_base } from "../mutable/base"
import { tableDataMutable_tree } from "../mutable/tree"
import {
    tableCellChildColumn,
    tableCellFooter,
    tableCellHeader,
    tableCellSummary,
    tableCellView,
    TableCell,
    TableDataTreeChildrenProvider,
    TableDataRelatedParams,
    TableDataRowKeyProvider,
    TableDataStyledParams,
    TableCellTree,
} from "../cell"
import {
    decorateRowStyle,
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataRowDecorator,
    TableDataRowRelatedDecorator,
    TableDataSummaryDecorator,
} from "../decorator"
import {
    inheritVerticalBorderStyle,
    TableDataFullStyle,
    TableDataHorizontalBorder,
    TableDataStyle,
    treePaddingStyle,
} from "../style"

export type TableDataTreeContent<M, R, C> = Readonly<{
    data: TableDataTreeChildrenProvider<M, R, C>
    key: TableDataRowKeyProvider<C>
    cells: TableCell<M, C>[]
}>
export function tableCell_tree<M, R, C>(content: TableDataTreeContent<M, R, C>): TableCellTree<M, R> {
    return new Cell(content)
}
class Cell<M, R, C> implements TableCellTree<M, R> {
    readonly type = "tree" as const

    content: TableDataTreeContent<M, R, C>
    mutable: Readonly<{
        core: TableDataMutable_base<R>
        tree: TableDataMutable_tree<R>
    }>

    constructor(content: TableDataTreeContent<M, R, C>) {
        this.content = content
        this.mutable = {
            core: tableDataMutable_base(),
            tree: tableDataMutable_tree(),
        }
    }

    view(params: TableDataParams<M>): TableDataView[] {
        return tableCellView(params, this.content.cells)
    }
    header(params: TableDataStyledParams<M>): TableDataHeader[] {
        const { style } = this.mutable.core.headerStyleMutable()
        return tableCellHeader(params, style, this.content.cells)
    }
    summary(params: TableDataStyledParams<M>): TableDataSummary[] {
        const { style } = this.mutable.core.summaryStyleMutable()
        return tableCellSummary(params, style, this.content.cells)
    }
    footer(params: TableDataStyledParams<M>): TableDataSummary[] {
        const { style } = this.mutable.core.footerStyleMutable()
        return tableCellFooter(params, style, this.content.cells)
    }
    column(params: TableDataRelatedParams<M, R>): TableDataColumnTree {
        const children = this.children(params)
        // 幅とスタイルを取得するために summary を構築する
        // summary が一番軽いだろうという判断
        const summaries = this.summary(params)
        return {
            type: "tree",
            children,
            length: length(summaries),
            height: height(children),
            style: this.paddingStyle(params.base, summaries),
        }

        function length(summaries: TableDataSummary[]): number {
            return summaries.reduce((acc, summary) => acc + summary.length, 0)
        }
        function height(rows: TableDataColumnRow[]): number {
            return Math.max(
                0,
                rows
                    .map((tree) =>
                        Math.max(
                            0,
                            ...tree.columns.map((column) => {
                                switch (column.type) {
                                    case "simple":
                                    case "expansion":
                                        return column.height

                                    case "tree":
                                        return height(column.children)
                                }
                            })
                        )
                    )
                    .reduce((acc, height) => acc + height, 0)
            )
        }
    }
    children(params: TableDataRelatedParams<M, R>): TableDataColumnRow[] {
        const { style } = this.mutable.core.columnStyleMutable()
        const rowMutable = this.mutable.tree.rowMutable()
        const { decorators } = this.mutable.core.columnMutable()

        const data = this.content.data(params.row, params.model)
        const dataLength = data.length
        return data.map((row, index) => {
            return {
                key: this.content.key(row),
                className: rowMutable.decorators.reduce(
                    (acc, decorator) => decorateRowStyle(acc, decorator(params.row)),
                    rowMutable.style
                ).className,
                columns: tableCellChildColumn(
                    params,
                    style,
                    decorators,
                    this.content.cells,
                    { row, last: index === dataLength - 1 }
                ),
            }
        })
    }
    paddingStyle(base: TableDataStyle, summaries: TableDataSummary[]): TableDataFullStyle {
        const { style } = this.mutable.core.columnStyleMutable()
        return treePaddingStyle(base, style.horizontalBorder, vertical())

        function vertical() {
            if (summaries.length === 0) {
                return inheritVerticalBorderStyle()
            }
            return {
                left: summaries[0].style.border.vertical.left,
                right: summaries[summaries.length - 1].style.border.vertical.right,
            }
        }
    }

    horizontalBorder(borders: TableDataHorizontalBorder[]): TableCellTree<M, R> {
        this.mutable.core.horizontalBorder(borders)
        return this
    }
    horizontalBorderRelated(borders: TableDataHorizontalBorderProvider<R>): TableCellTree<M, R> {
        this.mutable.core.horizontalBorderRelated(borders)
        return this
    }
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): TableCellTree<M, R> {
        this.mutable.core.horizontalBorder_header(borders)
        return this
    }
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): TableCellTree<M, R> {
        this.mutable.core.horizontalBorder_summary(borders)
        return this
    }
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): TableCellTree<M, R> {
        this.mutable.core.horizontalBorder_footer(borders)
        return this
    }

    decorateHeader(decorator: TableDataHeaderDecorator): TableCellTree<M, R> {
        this.mutable.core.decorateHeader(decorator)
        return this
    }
    decorateSummary(decorator: TableDataSummaryDecorator): TableCellTree<M, R> {
        this.mutable.core.decorateSummary(decorator)
        return this
    }
    decorateColumn(decorator: TableDataColumnDecorator): TableCellTree<M, R> {
        this.mutable.core.decorateColumn(decorator)
        return this
    }
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): TableCellTree<M, R> {
        this.mutable.core.decorateColumnRelated(decorator)
        return this
    }
    decorateRow(decorator: TableDataRowDecorator): TableCellTree<M, R> {
        this.mutable.tree.decorateRow(decorator)
        return this
    }
    decorateRowRelated(decorator: TableDataRowRelatedDecorator<R>): TableCellTree<M, R> {
        this.mutable.tree.decorateRowRelated(decorator)
        return this
    }
    decorateFooter(decorator: TableDataSummaryDecorator): TableCellTree<M, R> {
        this.mutable.core.decorateFooter(decorator)
        return this
    }
}
