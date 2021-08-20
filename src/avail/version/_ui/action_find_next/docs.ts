import { docsAction } from "../../../../../ui/vendor/getto-application/docs/helper"

export const docs_findNextVersion = docsAction("最新バージョンの確認", ({ item }) => [
    item("check", ["次のバージョンの存在確認"]),
    item("success", ["次のバージョンにアップデート"]),
])
