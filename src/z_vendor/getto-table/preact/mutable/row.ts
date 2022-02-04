import {
    TableDataHeaderRowMutable,
    TableDataMutable_row,
    TableDataStickyMutable,
    TableDataSummaryRowMutable,
} from "../mutable"
import { TableDataHeaderKeyProvider, TableDataKeyProvider } from "../core"
import { decorateRowStyle, TableDataRowDecorator } from "../decorator"

export function tableDataMutable_row(): TableDataMutable_row {
    return new Mutable()
}
class Mutable implements TableDataMutable_row {
    headerRow: TableDataHeaderRowMutable
    summaryRow: TableDataSummaryRowMutable
    footerRow: TableDataSummaryRowMutable
    sticky: TableDataStickyMutable

    constructor() {
        this.headerRow = {
            key: (i: number) => `__header_${i}`,
            style: {
                className: [],
            },
        }
        this.summaryRow = {
            key: () => "__summary",
            style: {
                className: [],
            },
        }
        this.footerRow = {
            key: () => "__footer",
            style: {
                className: [],
            },
        }
        this.sticky = {
            sticky: { type: "none" },
        }
    }

    headerRowMutable(): TableDataHeaderRowMutable {
        return this.headerRow
    }
    summaryRowMutable(): TableDataSummaryRowMutable {
        return this.summaryRow
    }
    footerRowMutable(): TableDataSummaryRowMutable {
        return this.footerRow
    }
    stickyMutable(): TableDataStickyMutable {
        return this.sticky
    }

    setHeaderKey(key: TableDataHeaderKeyProvider): void {
        this.headerRow = { ...this.headerRow, key }
    }
    setSummaryKey(key: TableDataKeyProvider): void {
        this.summaryRow = { ...this.summaryRow, key }
    }
    setFooterKey(key: TableDataKeyProvider): void {
        this.footerRow = { ...this.footerRow, key }
    }

    decorateHeaderRow(decorator: TableDataRowDecorator): void {
        this.headerRow = { ...this.headerRow, style: decorateRowStyle(this.headerRow.style, decorator) }
    }
    decorateSummaryRow(decorator: TableDataRowDecorator): void {
        this.summaryRow = {
            ...this.summaryRow,
            style: decorateRowStyle(this.summaryRow.style, decorator),
        }
    }
    decorateFooterRow(decorator: TableDataRowDecorator): void {
        this.footerRow = { ...this.footerRow, style: decorateRowStyle(this.footerRow.style, decorator) }
    }

    stickyTable(): void {
        this.sticky = { ...this.sticky, sticky: { type: "table" } }
    }
    stickyHeader(): void {
        this.sticky = { ...this.sticky, sticky: { type: "header" } }
    }
    stickyColumn(column: number): void {
        this.sticky = { ...this.sticky, sticky: { type: "column", column } }
    }
    stickyCross(column: number): void {
        this.sticky = { ...this.sticky, sticky: { type: "cross", column } }
    }
}
