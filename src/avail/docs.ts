import { DocsDescription } from "../z_vendor/getto-application/docs/data"

export const docs_avail: DocsDescription = {
    title: "アクセスの保証",
    descriptions: [
        {
            title: "停止許容時間",
            description: ["5分/10時間", "業務時間内を想定"],
        },
        {
            title: "レスポンス待ち時間",
            description: ["1秒", "これを超えるようならメッセージを表示するなどする"],
        },
    ],
}
