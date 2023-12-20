import { TableDataInherit } from "../cell"
import { TableDataKey } from "../core"
import { TableDataVisibleMutable } from "../mutable"

type VisibleParams = Partial<{ visibleKeys: readonly TableDataKey[] }>
export function isVisible(
    key: TableDataKey,
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
    if (params.visibleKeys) {
        return params.visibleKeys.includes(key)
    } else {
        return true
    }
}

export function initiallyVisibleCells(
    key: TableDataKey,
    { visibleType }: TableDataVisibleMutable,
): readonly TableDataKey[] {
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
