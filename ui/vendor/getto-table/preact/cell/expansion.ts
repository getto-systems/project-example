import { VNodeContent } from "../common"

import {
    TableDataCellKey,
    TableDataColumnExpansion,
    TableDataColumnSimple,
    TableDataHeaderExpansion,
    TableDataParams,
    TableDataSummaryExpansion,
    TableDataView,
    TableDataVisibleKeys,
} from "../core"

import { tableDataMutable_base } from "../mutable/base"
import { tableDataMutable_leaf } from "../mutable/leaf"
import { TableDataMutable_base, TableDataMutable_leaf } from "../mutable"
import {
    isVisibleKey,
    TableDataAlwaysVisible,
    TableCellExpansion,
    TableDataExpansionColumnContentProvider,
    TableDataInvisible,
    TableDataRelatedParams,
    TableDataStyledParams,
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

    isVisible(visibleKeys: TableDataVisibleKeys): boolean {
        const { visibleType: visible } = this.mutable.leaf.visibleMutable()
        return visible === "always" || isVisibleKey(this.key, visibleKeys)
    }

    verticalBorder(): TableDataVerticalBorderStyle {
        return this.mutable.leaf.verticalBorderMutable().border
    }

    view({ visibleKeys }: TableDataParams<M>): TableDataView | TableDataAlwaysVisible {
        const { visibleType: visible } = this.mutable.leaf.visibleMutable()
        if (visible === "always") {
            return { type: "always-visible" }
        }

        const { decorator } = this.mutable.leaf.viewMutable()
        return {
            type: "view",
            key: this.key,
            content: decorateContent(this.content.label, decorator),
            isVisible: this.isVisible(visibleKeys),
        }
    }
    header({
        visibleKeys,
        base,
        model,
    }: TableDataStyledParams<M>): TableDataHeaderExpansion | TableDataInvisible {
        if (!this.isVisible(visibleKeys)) {
            return { type: "invisible" }
        }
        const { style } = this.mutable.base.headerStyleMutable()
        return {
            type: "expansion",
            key: this.key,
            style: mergeVerticalBorder(extendStyle({ base, style }), this.verticalBorder()),
            content: this.content.header(this.content.label),
            length: this.length(model),
            height: 1,
        }
    }
    summary(params: TableDataStyledParams<M>): TableDataSummaryExpansion | TableDataInvisible {
        const { style } = this.mutable.base.summaryStyleMutable()
        return this.summaryContent(params, { style, content: content(this.content) })

        function content(content: TableDataExpansionContent<M, R>): TableDataSummaryProvider<M> {
            if ("summary" in content) {
                return { type: "content", content: content.summary }
            }
            return { type: "none" }
        }
    }
    column({
        visibleKeys,
        base,
        row,
        model,
    }: TableDataRelatedParams<M, R>): TableDataColumnExpansion | TableDataInvisible {
        if (!this.isVisible(visibleKeys)) {
            return { type: "invisible" }
        }
        const { style } = this.mutable.base.columnStyleMutable()
        const { decorators } = this.mutable.base.columnMutable()
        const length = this.length(model)
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
            columns: contents.map(
                (content, index): TableDataColumnSimple => {
                    return {
                        type: "simple",
                        key: [this.key, index].join(" "),
                        style: columnStyle,
                        content,
                        length: 1,
                        height: 1,
                    }
                },
            ),
        }
    }
    footer(params: TableDataStyledParams<M>): TableDataSummaryExpansion | TableDataInvisible {
        const { style } = this.mutable.base.footerStyleMutable()
        return this.summaryContent(params, { style, content: content(this.content) })

        function content(content: TableDataExpansionContent<M, R>): TableDataSummaryProvider<M> {
            if ("summary" in content) {
                return { type: "content", content: content.summary }
            }
            return { type: "none" }
        }
    }

    summaryContent(
        { visibleKeys, base, model }: TableDataStyledParams<M>,
        { style, content }: SummaryContentParams<M>,
    ): TableDataSummaryExpansion | TableDataInvisible {
        if (!this.isVisible(visibleKeys)) {
            return { type: "invisible" }
        }
        const shared = {
            key: this.key,
            style: mergeVerticalBorder(extendStyle({ base, style }), this.verticalBorder()),
            length: this.length(model),
        }
        switch (content.type) {
            case "none":
                return { type: "empty-expansion", ...shared }

            case "content":
                return {
                    type: "expansion",
                    ...shared,
                    content: content.content(model),
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
