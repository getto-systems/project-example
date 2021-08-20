import {
    docsDescription,
    docsDomain,
    docsModule,
    docsPurpose,
    docsSection,
    docsUsecase,
} from "../../ui/vendor/getto-application/docs/helper"

import {
    DocsSection,
    DocsUsecase,
    DocsUsecaseDescription,
} from "../../ui/vendor/getto-application/docs/data"
import { docs_notifyUnexpectedError } from "./unexpected_error/_ui/action_notify_unexpected_error/docs"

export const docs_avail = docsDomain<AvailUsecase, AvailAction, AvailData>(
    "保守・運用",
    ["業務で必要な時に使用できる", "業務に合ったコストで運用できる"],
    ["notifyUnexpectedError"],
    (name) => usecase[name],
)

const usecase = {
    notifyUnexpectedError: docsAvailUsecase(
        "notifyUnexpectedError",
        ["業務で必要な時に使用できる"],
        { action: ["notifyUnexpectedError"], data: [] },
    ),
} as const

const action = {
    notifyUnexpectedError: docs_notifyUnexpectedError,
} as const

const data = {} as const

export type AvailUsecase = keyof typeof usecase
export type AvailAction = keyof typeof action
export type AvailData = keyof typeof data

function docsAvailUsecase(
    title: AvailAction,
    purpose: string[],
    content: DocsUsecaseDescription<AvailAction, AvailData>,
): DocsUsecase<AvailAction, AvailData> {
    return docsUsecase(title, purpose, content, {
        toAction: (name) => action[name],
        toData: (name) => data[name],
    })
}

export const docs_avail_legacy: DocsSection[] = [
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
    docsSection("最新版にアップデート", [docsModule(["新しいバージョンを確認", "最新版に移行"])]),
    docsSection("発生したエラーを収集", [
        docsModule(["実行時エラーをサーバーに送信", "送信されたエラーを解析"]),
    ]),
]
