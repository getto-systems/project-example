import { VNodeContent } from "../common"

import {
    TableDataCellKey,
    TableDataColumnExpansion,
    TableDataColumnSimple,
    TableDataHeaderExpansion,
    TableDataSummaryExpansion,
    TableDataView,
} from "../core"

import { tableDataMutable_base } from "../mutable/base"
import { tableDataMutable_leaf } from "../mutable/leaf"
import { TableDataMutable_base, TableDataMutable_leaf } from "../mutable"
import {
    TableCellExpansion,
    TableDataExpansionColumnContentProvider,
    TableDataInvisible,
    TableDataRelatedParams,
    TableDataStyledParams,
    TableDataInherit,
} from "../cell"
import {
    decorateContent,
    decorateStyle,
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataContentDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataSummaryContentProvider,
    TableDataSummaryDecorator,
    TableDataSummaryProvider,
    TableDataViewDecorator,
} from "../decorator"
import {
    extendStyle,
    mergeVerticalBorder,
    TableDataHorizontalBorder,
    TableDataStyle,
    TableDataVerticalBorder,
    TableDataVerticalBorderStyle,
} from "../style"

import { initiallyVisibleCells, isVisible } from "./helper"

export type TableDataExpansionContent<M, R> =
    | TableDataExpansionContent_base<M, R>
    | (TableDataExpansionContent_base<M, R> & TableDataExpansionContent_summary<M>)
    | (TableDataExpansionContent_base<M, R> & TableDataExpansionContent_footer<M>)
    | (TableDataExpansionContent_base<M, R> &
          TableDataExpansionContent_summary<M> &
          TableDataExpansionContent_footer<M>)

type TableDataExpansionContent_base<M, R> = Readonly<{
    label: VNodeContent
    header: TableDataContentDecorator
    column: TableDataExpansionColumnContentProvider<R>
    length: TableDataExpansionLengthProvider<M>
}>
type TableDataExpansionContent_summary<M> = Readonly<{
    summary: TableDataSummaryContentProvider<M>
}>
type TableDataExpansionContent_footer<M> = Readonly<{ footer: TableDataSummaryContentProvider<M> }>

export function tableCell_expansion<M, R>(
    key: TableDataCellKey,
    content: { (key: TableDataCellKey): TableDataExpansionContent<M, R> },
): TableCellExpansion<M, R> {
    return new Cell(key, content(key))
}
class Cell<M, R> implements TableCellExpansion<M, R> {
    readonly type = "expansion" as const

    key: TableDataCellKey
    content: TableDataExpansionContent<M, R>
    mutable: Readonly<{
        base: TableDataMutable_base<R>
        leaf: TableDataMutable_leaf
    }>

    constructor(key: TableDataCellKey, content: TableDataExpansionContent<M, R>) {
        this.key = key
        this.content = content
        this.mutable = {
            base: tableDataMutable_base(),
            leaf: tableDataMutable_leaf(),
        }
    }

    length(model: M): number {
        return Math.max(1, this.content.length(model))
    }

    isVisible(inherit: TableDataInherit, params: TableDataStyledParams<M>): boolean {
        return isVisible(this.key, this.mutable.leaf.visibleMutable(), inherit, params)
    }

    verticalBorder(): TableDataVerticalBorderStyle {
        return this.mutable.leaf.verticalBorderMutable().border
    }

    initiallyVisibleCells(): TableDataCellKey[] {
        return initiallyVisibleCells(this.key, this.mutable.leaf.visibleMutable())
    }

    view(): TableDataView[] {
        const { visibleType } = this.mutable.leaf.visibleMutable()
        if (visibleType === "always") {
            // always visible なセルは view に含めない
            return []
        }

        const { decorator } = this.mutable.leaf.viewMutable()
        return [
            {
                key: this.key,
                content: decorateContent(this.content.label, decorator),
                isInitiallyVisible: visibleType !== "initially-hidden",
            },
        ]
    }
    header(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataHeaderExpansion | TableDataInvisible {
        if (!this.isVisible(inherit, params)) {
            return { type: "invisible" }
        }
        const { base, summary } = params
        const { style } = this.mutable.base.headerStyleMutable()
        return {
            type: "expansion",
            key: this.key,
            style: mergeVerticalBorder(extendStyle({ base, style }), this.verticalBorder()),
            content: this.content.header(this.content.label),
            length: this.length(summary),
            height: 1,
        }
    }
    summary(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummaryExpansion | TableDataInvisible {
        const { style } = this.mutable.base.summaryStyleMutable()
        return this.summaryContent(inherit, params, { style, content: content(this.content) })

        function content(content: TableDataExpansionContent<M, R>): TableDataSummaryProvider<M> {
            if ("summary" in content) {
                return { type: "content", content: content.summary }
            }
            return { type: "none" }
        }
    }
    column(
        inherit: TableDataInherit,
        params: TableDataRelatedParams<M, R>,
    ): TableDataColumnExpansion | TableDataInvisible {
        if (!this.isVisible(inherit, params)) {
            return { type: "invisible" }
        }
        const { base, row, summary } = params
        const { style } = this.mutable.base.columnStyleMutable()
        const { decorators } = this.mutable.base.columnMutable()
        const length = this.length(summary)
        const contents = this.content.column(row).slice(0, length)
        const columnStyle = mergeVerticalBorder(
            decorators.reduce(
                (acc, decorator) => decorateStyle(acc, decorator(row)),
                extendStyle({ base, style }),
            ),
            this.verticalBorder(),
        )
        return {
            type: "expansion",
            key: this.key,
            style: columnStyle,
            length,
            height: 1,
            columns: contents.map((content, index): TableDataColumnSimple => {
                return {
                    type: "simple",
                    key: [this.key, index].join(" "),
                    style: columnStyle,
                    content,
                    length: 1,
                    height: 1,
                }
            }),
        }
    }
    footer(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummaryExpansion | TableDataInvisible {
        const { style } = this.mutable.base.footerStyleMutable()
        return this.summaryContent(inherit, params, { style, content: content(this.content) })

        function content(content: TableDataExpansionContent<M, R>): TableDataSummaryProvider<M> {
            if ("summary" in content) {
                return { type: "content", content: content.summary }
            }
            return { type: "none" }
        }
    }

    summaryContent(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
        { style, content }: SummaryContentParams<M>,
    ): TableDataSummaryExpansion | TableDataInvisible {
        if (!this.isVisible(inherit, params)) {
            return { type: "invisible" }
        }
        const { base, summary } = params
        const shared = {
            key: this.key,
            style: mergeVerticalBorder(extendStyle({ base, style }), this.verticalBorder()),
            length: this.length(summary),
        }
        switch (content.type) {
            case "none":
                return { type: "empty-expansion", ...shared }

            case "content":
                return {
                    type: "expansion",
                    ...shared,
                    content: content.content(summary),
                }
        }
    }

    alwaysVisible(): TableCellExpansion<M, R> {
        this.mutable.leaf.alwaysVisible()
        return this
    }
    border(borders: TableDataVerticalBorder[]): TableCellExpansion<M, R> {
        this.mutable.leaf.border(borders)
        return this
    }

    horizontalBorder(borders: TableDataHorizontalBorder[]): TableCellExpansion<M, R> {
        this.mutable.base.horizontalBorder(borders)
        return this
    }
    horizontalBorderRelated(
        borders: TableDataHorizontalBorderProvider<R>,
    ): TableCellExpansion<M, R> {
        this.mutable.base.horizontalBorderRelated(borders)
        return this
    }
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): TableCellExpansion<M, R> {
        this.mutable.base.horizontalBorder_header(borders)
        return this
    }
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): TableCellExpansion<M, R> {
        this.mutable.base.horizontalBorder_summary(borders)
        return this
    }
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): TableCellExpansion<M, R> {
        this.mutable.base.horizontalBorder_footer(borders)
        return this
    }

    decorateView(decorator: TableDataViewDecorator): TableCellExpansion<M, R> {
        this.mutable.leaf.decorateView(decorator)
        return this
    }
    decorateHeader(decorator: TableDataHeaderDecorator): TableCellExpansion<M, R> {
        this.mutable.base.decorateHeader(decorator)
        return this
    }
    decorateSummary(decorator: TableDataSummaryDecorator): TableCellExpansion<M, R> {
        this.mutable.base.decorateSummary(decorator)
        return this
    }
    decorateColumn(decorator: TableDataColumnDecorator): TableCellExpansion<M, R> {
        this.mutable.base.decorateColumn(decorator)
        return this
    }
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): TableCellExpansion<M, R> {
        this.mutable.base.decorateColumnRelated(decorator)
        return this
    }
    decorateFooter(decorator: TableDataSummaryDecorator): TableCellExpansion<M, R> {
        this.mutable.base.decorateFooter(decorator)
        return this
    }
}

type SummaryContentParams<M> = Readonly<{
    style: TableDataStyle
    content: TableDataSummaryProvider<M>
}>

interface TableDataExpansionLengthProvider<S> {
    (model: S): number
}
