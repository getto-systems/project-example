import { docs_notifyUnexpectedError } from "./unexpected_error/notify/docs"
import { docs_findNextVersion } from "./version/find_next/docs"

import { DocsAction, DocsDomain } from "../z_vendor/getto-application/docs/data"

const docs_guaranteeOnTimeAccess: DocsAction = {
    title: "業務時間内のアクセスを保証",
    action: [
        {
            type: "check",
            check: ["停止許容時間", "5分/10h (業務時間内)"],
            help: ["業務時間外の停止は連絡後に実施"],
        },
        {
            type: "check",
            check: ["レスポンスの最大待ち時間", "1秒"],
            help: ["処理に時間がかかる場合は完了時に通知"],
        },
    ],
    data: [],
}

const docs_server: DocsAction = {
    title: "サーバー構成",
    action: [
        {
            type: "check",
            check: ["コストを抑えた構成", "冗長構成"],
            help: ["Google Cloud Run", "K8s", "Cloud SQL", "AWS RDS"],
        },
    ],
    data: [],
}

export const docs_avail: DocsDomain = {
    path: "avail",
    title: "保守・運用",
    usecase: [
        {
            path: "unexpected-error/notify",
            title: docs_notifyUnexpectedError.title,
            purpose: ["業務で必要な時に使用できる"],
            action: [docs_notifyUnexpectedError],
        },
        {
            path: "next-version/find",
            title: docs_findNextVersion.title,
            purpose: ["業務で必要な時に使用できる"],
            action: [docs_findNextVersion],
        },
        {
            path: "server/operate",
            title: docs_guaranteeOnTimeAccess.title,
            purpose: ["業務で必要な時に使用できる"],
            action: [docs_guaranteeOnTimeAccess],
        },
        {
            path: "server/run",
            title: docs_server.title,
            purpose: ["業務で必要な時に使用できる", "業務に合ったコストで運用できる"],
            action: [docs_server],
        },
    ],
}
