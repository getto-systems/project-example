import {
    docsAction,
    docsDescription,
    docsDomain,
    docsModule,
    docsPath,
    docsPurpose,
    docsSection,
    docsUsecase,
} from "../../ui/vendor/getto-application/docs/helper"

import { docs_notifyUnexpectedError } from "./unexpected_error/_ui/action_notify/docs"
import { docs_findNextVersion } from "./version/_ui/action_find_next/docs"

import {
    DocsSection,
    DocsUsecase,
    DocsUsecaseDescription,
} from "../../ui/vendor/getto-application/docs/data"

export const docs_avail = docsDomain<AvailUsecase, AvailAction, AvailData>(
    "保守・運用",
    ["業務で必要な時に使用できる", "業務に合ったコストで運用できる"],
    ["guaranteeOnTimeAccess", "notifyUnexpectedError", "findNextVersion", "server"],
    (name) => usecase[name],
)

const usecase = {
    guaranteeOnTimeAccess: docsAvailUsecase(
        "guaranteeOnTimeAccess",
        ["業務で必要な時に使用できる"],
        { action: ["guaranteeOnTimeAccess"], data: [] },
    ),
    notifyUnexpectedError: docsAvailUsecase(
        "notifyUnexpectedError",
        ["業務で必要な時に使用できる"],
        { action: ["notifyUnexpectedError"], data: [] },
    ),
    findNextVersion: docsAvailUsecase("findNextVersion", ["業務で必要な時に使用できる"], {
        action: ["findNextVersion"],
        data: [],
    }),
    server: docsAvailUsecase(
        "server",
        ["業務で必要な時に使用できる", "業務に合ったコストで運用できる"],
        { action: ["server"], data: [] },
    ),
} as const

const action = {
    guaranteeOnTimeAccess: docsAction("業務時間内のアクセスを保証", ({ item }) => [
        item("check", ["停止許容時間", "5分/10h (業務時間内)"], ["業務時間外の停止は連絡後に実施"]),
        item(
            "check",
            ["レスポンスの最大待ち時間", "1秒"],
            ["処理に時間がかかる場合は完了時に通知"],
        ),
    ]),
    notifyUnexpectedError: docs_notifyUnexpectedError,
    findNextVersion: docs_findNextVersion,
    server: docsAction("サーバー構成", ({ item }) => [
        item(
            "check",
            ["コストを抑えた構成", "冗長構成"],
            ["Google Cloud Run", "K8s", "Cloud SQL", "AWS RDS"],
        ),
    ]),
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
    return docsUsecase(docsPath(title), title, purpose, content, {
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
