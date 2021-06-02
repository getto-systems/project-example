import { VNodeContent } from "../common"

import {
    TableDataCellKey,
    TableDataColumn,
    TableDataHeader,
    TableDataHeaderGroup,
    TableDataParams,
    TableDataSummary,
    TableDataView,
} from "../core"

import { tableDataMutable_base } from "../mutable/base"
import { tableDataMutable_group } from "../mutable/group"
import { TableDataMutable_base, TableDataMutable_group } from "../mutable"
import {
    TableCell,
    TableCellGroup,
    TableDataRelatedParams,
    TableDataStyledParams,
    tableCellView,
    tableCellSummary,
    tableCellColumn,
    tableCellHeader,
    tableCellFooter,
    TableDataInvisible,
} from "../cell"
import {
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataGroupDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataSummaryDecorator,
    TableDataViewDecorator,
} from "../decorator"
import {
    baseGroupStyle,
    baseGroupMemberStyle,
    extendStyle,
    mergeVerticalBorder,
    TableDataHorizontalBorder,
} from "../style"

export type TableDataGroupContent<M, R> = Readonly<{
    key: TableDataCellKey
    header: VNodeContent
    cells: TableCell<M, R>[]
}>
export function tableCell_group<M, R>(content: TableDataGroupContent<M, R>): TableCellGroup<M, R> {
    return new Cell(content)
}
class Cell<M, R> implements TableCellGroup<M, R> {
    readonly type = "group" as const

    content: TableDataGroupContent<M, R>
    mutable: Readonly<{
        core: TableDataMutable_base<R>
        group: TableDataMutable_group
    }>

    constructor(content: TableDataGroupContent<M, R>) {
        this.content = content
        this.mutable = {
            core: tableDataMutable_base(),
            group: tableDataMutable_group(),
        }
    }

    view(params: TableDataParams<M>): TableDataView[] {
        return tableCellView(params, this.content.cells)
    }
    header(params: TableDataStyledParams<M>): TableDataHeaderGroup | TableDataInvisible {
        const children = this.children(params)
        if (children.length === 0) {
            return { type: "invisible" }
        }
        const { style } = this.mutable.group.groupStyleMutable()
        return {
            type: "group",
            key: this.content.key,
            style: mergeVerticalBorder(
                extendStyle({
                    base: baseGroupStyle(params.base),
                    style,
                }),
                verticalBorder(children[0], children[children.length - 1]),
            ),
            content: this.content.header,
            children,
            length: length(children),
            height: height(children) + 1,
        }

        function verticalBorder(first: TableDataHeader, last: TableDataHeader) {
            return {
                left: first.style.border.vertical.left,
                right: last.style.border.vertical.right,
            }
        }
        function length(headers: TableDataHeader[]): number {
            return headers.reduce((acc, header) => {
                switch (header.type) {
                    case "simple":
                    case "expansion":
                        return acc + header.length

                    case "group":
                        return acc + length(header.children)
                }
            }, 0)
        }
        function height(headers: TableDataHeader[]): number {
            return Math.max(
                0,
                ...headers.map((header) => {
                    switch (header.type) {
                        case "simple":
                        case "expansion":
                            return header.height

                        case "group":
                            return height(header.children) + 1
                    }
                }),
            )
        }
    }
    children(params: TableDataStyledParams<M>): TableDataHeader[] {
        const { style } = this.mutable.core.headerStyleMutable()
        return tableCellHeader(
            { ...params, base: baseGroupMemberStyle(params.base) },
            style,
            this.content.cells,
        )
    }
    summary(params: TableDataStyledParams<M>): TableDataSummary[] {
        const { style } = this.mutable.core.summaryStyleMutable()
        return tableCellSummary(params, style, this.content.cells)
    }
    footer(params: TableDataStyledParams<M>): TableDataSummary[] {
        const { style } = this.mutable.core.footerStyleMutable()
        return tableCellFooter(params, style, this.content.cells)
    }
    column(params: TableDataRelatedParams<M, R>): TableDataColumn[] {
        const { style } = this.mutable.core.columnStyleMutable()
        const { decorators } = this.mutable.core.columnMutable()
        return tableCellColumn(params, style, decorators, this.content.cells)
    }

    horizontalBorder(borders: TableDataHorizontalBorder[]): TableCellGroup<M, R> {
        this.mutable.core.horizontalBorder(borders)
        return this
    }
    horizontalBorderRelated(borders: TableDataHorizontalBorderProvider<R>): TableCellGroup<M, R> {
        this.mutable.core.horizontalBorderRelated(borders)
        return this
    }
    horizontalBorder_group(borders: TableDataHorizontalBorder[]): TableCellGroup<M, R> {
        this.mutable.group.horizontalBorder_group(borders)
        return this
    }
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): TableCellGroup<M, R> {
        this.mutable.core.horizontalBorder_header(borders)
        return this
    }
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): TableCellGroup<M, R> {
        this.mutable.core.horizontalBorder_summary(borders)
        return this
    }
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): TableCellGroup<M, R> {
        this.mutable.core.horizontalBorder_footer(borders)
        return this
    }

    decorateView(decorator: TableDataViewDecorator): TableCellGroup<M, R> {
        this.mutable.group.decorateView(decorator)
        return this
    }
    decorateGroup(decorator: TableDataGroupDecorator): TableCellGroup<M, R> {
        this.mutable.group.decorateGroup(decorator)
        return this
    }
    decorateHeader(decorator: TableDataHeaderDecorator): TableCellGroup<M, R> {
        this.mutable.core.decorateHeader(decorator)
        return this
    }
    decorateSummary(decorator: TableDataSummaryDecorator): TableCellGroup<M, R> {
        this.mutable.core.decorateSummary(decorator)
        return this
    }
    decorateColumn(decorator: TableDataColumnDecorator): TableCellGroup<M, R> {
        this.mutable.core.decorateColumn(decorator)
        return this
    }
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): TableCellGroup<M, R> {
        this.mutable.core.decorateColumnRelated(decorator)
        return this
    }
    decorateFooter(decorator: TableDataSummaryDecorator): TableCellGroup<M, R> {
        this.mutable.core.decorateFooter(decorator)
        return this
    }
}
