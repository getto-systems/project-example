import { TableDataColumnMutable, TableDataStyleMutable, TableDataMutable_base } from "../mutable"
import {
    decorateStyle,
    horizontalBorder,
    TableDataColumnDecorator,
    TableDataColumnRelatedDecorator,
    TableDataHeaderDecorator,
    TableDataHorizontalBorderProvider,
    TableDataSummaryDecorator,
} from "../decorator"
import {
    defaultColumnStyle,
    defaultFooterStyle,
    defaultHeaderStyle,
    defaultSummaryStyle,
    inheritStyle,
    TableDataHorizontalBorder,
} from "../style"

export function tableDataMutable_base<R>(): TableDataMutable_base<R> {
    return new Mutable()
}
export function tableDataMutable_default<R>(): TableDataMutable_base<R> {
    const mutable = new Mutable<R>()
    mutable.useDefaultStyle()
    return mutable
}
class Mutable<R> implements TableDataMutable_base<R> {
    headerStyle: TableDataStyleMutable
    summaryStyle: TableDataStyleMutable
    columnStyle: TableDataStyleMutable
    footerStyle: TableDataStyleMutable
    column: TableDataColumnMutable<R>

    constructor() {
        this.headerStyle = {
            style: inheritStyle(),
        }
        this.summaryStyle = {
            style: inheritStyle(),
        }
        this.columnStyle = {
            style: inheritStyle(),
        }
        this.footerStyle = {
            style: inheritStyle(),
        }
        this.column = {
            decorators: [],
        }
    }

    useDefaultStyle(): void {
        this.headerStyle = {
            style: defaultHeaderStyle(),
        }
        this.summaryStyle = {
            style: defaultSummaryStyle(),
        }
        this.columnStyle = {
            style: defaultColumnStyle(),
        }
        this.footerStyle = {
            style: defaultFooterStyle(),
        }
    }

    headerStyleMutable(): TableDataStyleMutable {
        return this.headerStyle
    }
    summaryStyleMutable(): TableDataStyleMutable {
        return this.summaryStyle
    }
    columnStyleMutable(): TableDataStyleMutable {
        return this.columnStyle
    }
    footerStyleMutable(): TableDataStyleMutable {
        return this.footerStyle
    }
    columnMutable(): TableDataColumnMutable<R> {
        return this.column
    }

    horizontalBorder(borders: TableDataHorizontalBorder[]): void {
        this.decorateColumn(horizontalBorder(borders))
    }
    horizontalBorderRelated(borders: TableDataHorizontalBorderProvider<R>): void {
        this.decorateColumnRelated((row: R) => horizontalBorder(borders(row)))
    }
    horizontalBorder_header(borders: TableDataHorizontalBorder[]): void {
        this.decorateHeader(horizontalBorder(borders))
    }
    horizontalBorder_summary(borders: TableDataHorizontalBorder[]): void {
        this.decorateSummary(horizontalBorder(borders))
    }
    horizontalBorder_footer(borders: TableDataHorizontalBorder[]): void {
        this.decorateFooter(horizontalBorder(borders))
    }

    decorateHeader(decorator: TableDataHeaderDecorator): void {
        this.headerStyle = {
            ...this.headerStyle,
            style: decorateStyle(this.headerStyle.style, decorator),
        }
    }
    decorateSummary(decorator: TableDataSummaryDecorator): void {
        this.summaryStyle = {
            ...this.summaryStyle,
            style: decorateStyle(this.summaryStyle.style, decorator),
        }
    }
    decorateColumn(decorator: TableDataColumnDecorator): void {
        this.columnStyle = {
            ...this.columnStyle,
            style: decorateStyle(this.columnStyle.style, decorator),
        }
    }
    decorateColumnRelated(decorator: TableDataColumnRelatedDecorator<R>): void {
        this.column = {
            ...this.column,
            decorators: [...this.column.decorators, decorator],
        }
    }
    decorateFooter(decorator: TableDataSummaryDecorator): void {
        this.footerStyle = {
            ...this.footerStyle,
            style: decorateStyle(this.footerStyle.style, decorator),
        }
    }
}
