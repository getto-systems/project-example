import { TableDataMutable_tree, TableDataRowMutable } from "../mutable"
import { decorateRowStyle, TableDataRowDecorator, TableDataRowRelatedDecorator } from "../decorator"
import { emptyRowStyle } from "../style"

export function tableDataMutable_tree<R>(): TableDataMutable_tree<R> {
    return new Mutable()
}
class Mutable<R> implements TableDataMutable_tree<R> {
    row: TableDataRowMutable<R>

    constructor() {
        this.row = {
            style: emptyRowStyle(),
            decorators: [],
        }
    }

    rowMutable(): TableDataRowMutable<R> {
        return this.row
    }

    decorateRow(decorator: TableDataRowDecorator): void {
        this.row = { ...this.row, style: decorateRowStyle(this.row.style, decorator) }
    }
    decorateRowRelated(decorator: TableDataRowRelatedDecorator<R>): void {
        this.row = { ...this.row, decorators: [...this.row.decorators, decorator] }
    }
}
