import {
    docsDescription,
    docsModule,
    docsPurpose,
    docsSection,
} from "../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../ui/vendor/getto-application/docs/data"

export const docs_avail: DocsSection[] = [
    docsSection("保守・運用", [
        docsPurpose(["業務で必要な時に使用できる", "業務に合ったコストで運用できる"]),
        docsModule([
            "業務時間内のアクセスを保証",
            "コストがかかりすぎない構成",
            "最新版にアップデート",
            "発生したエラーを収集",
        ]),
    ]),
]

export const docs_avail_detail: DocsSection[] = [
    docsSection("業務時間内のアクセスを保証", [
        docsDescription([
            {
                title: "停止許容時間",
                body: ["5分/10h (業務時間内)"],
                help: ["業務時間外の停止は連絡後に実施"],
            },
            {
                title: "レスポンスの最大待ち時間",
                body: ["1秒"],
                help: ["処理に時間がかかる場合は完了時に通知"],
            },
        ]),
    ]),
    docsSection("コストがかかりすぎない構成", [
        docsDescription([
            {
                title: "アプリケーションサーバー",
                body: ["Google Cloud Run", "K8s"],
                help: ["冗長構成のサーバーにデプロイ"],
            },
            {
                title: "データベースサーバー",
                body: ["Google SQL", "amazon RDS"],
                help: ["マネージドな SQL サーバー"],
            },
        ]),
    ]),
    docsSection("最新版にアップデート", [
        docsModule(["新しいバージョンを確認", "最新版に移行"]),
    ]),
    docsSection("発生したエラーを収集", [
        docsModule(["実行時エラーをサーバーに送信", "送信されたエラーを解析"]),
    ]),
]
