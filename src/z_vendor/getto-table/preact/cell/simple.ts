import { PreactContent } from "../common"

import {
    TableDataKey,
    TableDataColumnSimple,
    TableDataHeaderSimple,
    TableDataSummarySimple,
    TableDataCell,
} from "../core"

import { tableDataMutable_base } from "../mutable/base"
import { tableDataMutable_leaf } from "../mutable/leaf"
import { TableDataMutable_base, TableDataMutable_leaf } from "../mutable"
import {
    TableDataColumnContentProvider,
    TableDataInvisible,
    TableDataRelatedParams,
    TableCellSimple,
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

export type TableDataSimpleContent<M, R> = Readonly<{
    label: PreactContent
    header: TableDataContentDecorator
    column: TableDataColumnContentProvider<R>
}> &
    Partial<{
        summary: TableDataSummaryContentProvider<M>
        footer: TableDataSummaryContentProvider<M>
    }>

export function tableCell<K extends TableDataKey, M, R>(
    key: K,
    content: { (key: K): TableDataSimpleContent<M, R> },
): TableCellSimple<M, R> {
    return new Cell(key, content(key))
}
class Cell<M, R> implements TableCellSimple<M, R> {
    readonly type = "simple" as const

    key: TableDataKey
    content: TableDataSimpleContent<M, R>
    mutable: Readonly<{
        base: TableDataMutable_base<R>
        leaf: TableDataMutable_leaf
    }>

    constructor(key: TableDataKey, content: TableDataSimpleContent<M, R>) {
        this.key = key
        this.content = content
        this.mutable = {
            base: tableDataMutable_base(),
            leaf: tableDataMutable_leaf(),
        }
    }

    isVisible(inherit: TableDataInherit, params: TableDataStyledParams<M>): boolean {
        return isVisible(this.key, this.mutable.leaf.visibleMutable(), inherit, params)
    }

    verticalBorder(): TableDataVerticalBorderStyle {
        return this.mutable.leaf.verticalBorderMutable().border
    }

    initiallyVisibleCells(): readonly TableDataKey[] {
        return initiallyVisibleCells(this.key, this.mutable.leaf.visibleMutable())
    }

    view(): readonly TableDataCell[] {
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
    ): TableDataHeaderSimple | TableDataInvisible {
        if (!this.isVisible(inherit, params)) {
            return { type: "invisible" }
        }
        const { base } = params
        const { style } = this.mutable.base.headerStyleMutable()
        return {
            type: "simple",
            key: this.key,
            style: mergeVerticalBorder(extendStyle({ base, style }), this.verticalBorder()),
            content: this.content.header(this.content.label),
            length: 1,
            height: 1,
        }
    }
    summary(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummarySimple | TableDataInvisible {
        const { style } = this.mutable.base.summaryStyleMutable()
        return this.summaryContent(inherit, params, { style, content: content(this.content) })

        function content(content: TableDataSimpleContent<M, R>): TableDataSummaryProvider<M> {
            if (content.summary) {
                return { type: "content", content: content.summary }
            }
            return { type: "none" }
        }
    }
    column(
        inherit: TableDataInherit,
        params: TableDataRelatedParams<M, R>,
    ): TableDataColumnSimple | TableDataInvisible {
        if (!this.isVisible(inherit, params)) {
            return { type: "invisible" }
        }
        const { base, row } = params
        const { style } = this.mutable.base.columnStyleMutable()
        const { decorators } = this.mutable.base.columnMutable()
        return {
            type: "simple",
            key: this.key,
            style: mergeVerticalBorder(
                decorators.reduce(
                    (acc, decorator) => decorateStyle(acc, decorator(row)),
                    extendStyle({ base, style }),
                ),
                this.verticalBorder(),
            ),
            content: this.content.column(row),
            length: 1,
            height: 1,
        }
    }
    footer(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
    ): TableDataSummarySimple | TableDataInvisible {
        const { style } = this.mutable.base.footerStyleMutable()
        return this.summaryContent(inherit, params, { style, content: content(this.content) })

        function content(content: TableDataSimpleContent<M, R>): TableDataSummaryProvider<M> {
            if (content.footer) {
                return { type: "content", content: content.footer }
            }
            return { type: "none" }
        }
    }
    summaryContent(
        inherit: TableDataInherit,
        params: TableDataStyledParams<M>,
        { style, content }: SummaryContentParams<M>,
    ): TableDataSummarySimple | TableDataInvisible {
        if (!this.isVisible(inherit, params)) {
            return { type: "invisible" }
        }
        const { base, summary } = params
        const shared = {
            key: this.key,
            style: mergeVerticalBorder(extendStyle({ base, style }), this.verticalBorder()),
            length: 1 as const,
        }
        switch (content.type) {
            case "none":
                return { type: "empty", ...shared }

            case "content":
                return {
                    type: "simple",
                    ...shared,
                    content: content.content(summary),
                }
        }
    }

    alwaysVisible(): TableCellSimple<M, R> {
        this.mutable.leaf.alwaysVisible()
        return this
    }
    initiallyHidden(): TableCellSimple<M, R> {
        this.mutable.leaf.initiallyHidden()
        return this
    }
    border(borders: readonly TableDataVerticalBorder[]): TableCellSimple<M, R> {
        this.mutable.leaf.border(borders)
        return this
    }

    horizontalBorder(borders: readonly TableDataHorizontalBorder[]): TableCellSimple<M, R> {
        this.mutable.base.horizontalBorder(borders)
        return this
    }
    horizontalBorderRelated(borders: TableDataHorizontalBorderProvider<R>): TableCellSimple<M, R> {
        this.mutable.base.horizontalBorderRelated(borders)
        return this
    }
    horizontalBorder_header(borders: readonly TableDataHorizontalBorder[]): TableCellSimple<M, R> {
        this.mutable.base.horizontalBorder_header(borders)
        return this
    }
    horizontalBorder_summary(borders: readonly TableDataHorizontalBorder[]): TableCellSimple<M, R> {
        this.mutable.base.horizontalBorder_summary(borders)
        return this
    }
    horizontalBorder_footer(borders: readonly TableDataHorizontalBorder[]): TableCellSimple<M, R> {
        this.mutable.base.horizontalBorder_footer(borders)
        return this
    }

    decorateView(decorator: TableDataViewDecorator): TableCellSimple<M, R> {
        this.mutable.leaf.decorateView(decorator)
        return this
    }
    decorateHeader(decorator: TableDataHeaderDecorator): TableCellSimple<M, R> {
        this.mutable.base.decorateHeader(decorator)
        return this
    }
    decorateSummary(decorator: TableDataSummaryDecorator): TableCellSimple<M, R> {
        this.mutable.base.decorateSummary(decorator)
        return this
    }
    decorateColumn(decorator: TableDataColumnDecorator): TableCellSimple<M, R> {
        this.mutable.base.decorateColumn(decorator)
        return this
    }
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): TableCellSimple<M, R> {
        this.mutable.base.decorateColumnRelated(decorator)
        return this
    }
    decorateFooter(decorator: TableDataSummaryDecorator): TableCellSimple<M, R> {
        this.mutable.base.decorateFooter(decorator)
        return this
    }
}

type SummaryContentParams<M> = Readonly<{
    style: TableDataStyle
    content: TableDataSummaryProvider<M>
}>
