import { VNodeContent } from "./common"

import { decorateAlign } from "./decorator/align"
import { decorateHorizontalBorder } from "./decorator/border"
import {
    TableDataAlign,
    TableDataAlignStyle,
    TableDataClassName,
    TableDataHorizontalBorder,
    TableDataHorizontalBorderStyle,
    TableDataRowStyle,
    TableDataStyle,
} from "./style"

export type TableDataViewDecorator = TableDataContentDecoratorProvider
export type TableDataHeaderDecorator = TableDataStyleDecorator
export type TableDataGroupDecorator = TableDataStyleDecorator
export type TableDataSummaryDecorator = TableDataStyleDecorator
export type TableDataColumnDecorator = TableDataStyleDecorator
export type TableDataColumnRelatedDecorator<R> = { (row: R): TableDataStyleDecorator }
export type TableDataRowDecorator = TableDataRowStyleDecorator
export type TableDataRowRelatedDecorator<R> = { (row: R): TableDataRowStyleDecorator }

export type TableDataContentDecoratorProvider =
    | Readonly<{ type: "none" }>
    | Readonly<{ type: "decorate"; decorator: TableDataContentDecorator }>

export interface TableDataContentDecorator {
    (label: VNodeContent): VNodeContent
}

export interface TableDataHorizontalBorderProvider<R> {
    (row: R): TableDataHorizontalBorder[]
}

export type TableDataSummaryProvider<M> =
    | Readonly<{ type: "none" }>
    | Readonly<{ type: "content"; content: TableDataSummaryContentProvider<M> }>

export interface TableDataContentProvider {
    (): VNodeContent
}
export interface TableDataSummaryContentProvider<M> {
    (model: M): VNodeContent
}

export type TableDataStyleDecorator =
    | TableDataStyleDecorator_none
    | TableDataStyleDecorator_border
    | TableDataStyleDecorator_align
    | TableDataStyleDecorator_className

export type TableDataRowStyleDecorator =
    | TableDataStyleDecorator_none
    | TableDataStyleDecorator_className

type TableDataStyleDecorator_none = Readonly<{ type: "none" }>
type TableDataStyleDecorator_border = Readonly<{
    type: "border"
    decorator: Decorator<TableDataHorizontalBorderStyle>
}>
type TableDataStyleDecorator_align = Readonly<{
    type: "align"
    decorator: Decorator<TableDataAlignStyle>
}>
type TableDataStyleDecorator_className = Readonly<{
    type: "className"
    className: TableDataClassName
}>

export function decorateStyle(
    style: TableDataStyle,
    decorator: TableDataStyleDecorator,
): TableDataStyle {
    switch (decorator.type) {
        case "none":
            return style

        case "border":
            return {
                ...style,
                horizontalBorder: decorator.decorator(style.horizontalBorder),
            }

        case "align":
            return { ...style, align: decorator.decorator(style.align) }

        case "className":
            return { ...style, className: [...style.className, ...decorator.className] }
    }
}
export function decorateRowStyle(
    style: TableDataRowStyle,
    decorator: TableDataRowStyleDecorator,
): TableDataRowStyle {
    switch (decorator.type) {
        case "none":
            return style

        case "className":
            return { ...style, className: [...style.className, ...decorator.className] }
    }
}

export function decorateContent(
    content: VNodeContent,
    decorator: TableDataContentDecoratorProvider,
): VNodeContent {
    switch (decorator.type) {
        case "none":
            return content

        case "decorate":
            return decorator.decorator(content)
    }
}

export const decorateNone: TableDataStyleDecorator_none = { type: "none" }

export function horizontalBorder(
    borders: TableDataHorizontalBorder[],
): TableDataStyleDecorator_border {
    return { type: "border", decorator: decorateHorizontalBorder(borders) }
}
export function tableAlign(aligns: TableDataAlign[]): TableDataStyleDecorator_align {
    return { type: "align", decorator: decorateAlign(aligns) }
}
export function tableClassName(className: TableDataClassName): TableDataStyleDecorator_className {
    return { type: "className", className }
}

interface Decorator<T> {
    (base: T): T
}
