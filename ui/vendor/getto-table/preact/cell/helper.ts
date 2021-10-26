import { TableDataInherit } from "../cell"
import { TableDataCellKey } from "../core"
import { TableDataVisibleMutable } from "../mutable"

type VisibleParams =
    | {
          // no props
      }
    | Readonly<{ visibleKeys: readonly TableDataCellKey[] }>
export function isVisible(
    key: TableDataCellKey,
    { visibleType }: TableDataVisibleMutable,
    inherit: TableDataInherit,
    params: VisibleParams,
): boolean {
    if (inherit.isInMultipart) {
        // multipart の場合、データ取得前にセルを特定できない
        // データを取得してから visible keys の指定をするでは遅いので、
        // multipart の配下なら常に visible とする
        return true
    }
    if (visibleType === "always") {
        return true
    }
    if ("visibleKeys" in params) {
        return params.visibleKeys.includes(key)
    } else {
        return true
    }
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
