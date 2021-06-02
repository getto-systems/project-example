import {
    TableDataMutable_leaf,
    TableDataVerticalBorderMutable,
    TableDataViewMutable,
    TableDataVisibleMutable,
} from "../mutable"
import { decorateVerticalBorder } from "../decorator/border"
import { TableDataViewDecorator } from "../decorator"
import { inheritVerticalBorderStyle, TableDataVerticalBorder } from "../style"

export function tableDataMutable_leaf(): TableDataMutable_leaf {
    return new Mutable()
}
class Mutable implements TableDataMutable_leaf {
    visible: TableDataVisibleMutable
    view: TableDataViewMutable
    verticalBorder: TableDataVerticalBorderMutable

    constructor() {
        this.visible = {
            visibleType: "normal",
        }
        this.view = {
            decorator: { type: "none" },
        }
        this.verticalBorder = {
            border: inheritVerticalBorderStyle(),
        }
    }

    visibleMutable(): TableDataVisibleMutable {
        return this.visible
    }
    viewMutable(): TableDataViewMutable {
        return this.view
    }
    verticalBorderMutable(): TableDataVerticalBorderMutable {
        return this.verticalBorder
    }

    alwaysVisible(): void {
        this.visible = { ...this.visible, visibleType: "always" }
    }
    border(borders: TableDataVerticalBorder[]): void {
        this.verticalBorder = {
            ...this.verticalBorder,
            border: decorateVerticalBorder(borders)(this.verticalBorder.border),
        }
    }

    decorateView(decorator: TableDataViewDecorator): void {
        this.view = { ...this.view, decorator }
    }
}
