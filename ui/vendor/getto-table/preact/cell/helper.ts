import { TableDataInherit } from "../cell"
import { TableDataCellKey } from "../core"
import { TableDataVisibleMutable } from "../mutable"

export function isVisible(
    key: TableDataCellKey,
    { visibleType }: TableDataVisibleMutable,
    inherit: TableDataInherit,
    visibleKeys: readonly TableDataCellKey[],
): boolean {
    if (inherit.isInMultipart) {
        // multipart の場合、データ取得前にセルを特定できない
        // データを取得してから visible keys の指定をするでは遅いので、
        // multipart の配下なら常に visible とする
        return true
    }
    return visibleType === "always" || visibleKeys.includes(key)
}

export function initiallyVisibleCells(
    key: TableDataCellKey,
    { visibleType }: TableDataVisibleMutable,
): TableDataCellKey[] {
    switch (visibleType) {
        case "always":
            // 常に表示されるセルは初期表示セルの中に含めない
            return []

        case "initially-hidden":
            return []

        case "normal":
            return [key]
    }
}
