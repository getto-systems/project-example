import { DocsAction } from "../../../z_vendor/getto-application/docs/data"

export const docs_findNextVersion: DocsAction = {
    title: "最新バージョンの確認",
    action: [
        {
            type: "check",
            check: ["次のバージョンの存在確認"],
        },
        {
            type: "success",
            action: ["次のバージョンにアップデート"],
        },
    ],
    data: [],
}
