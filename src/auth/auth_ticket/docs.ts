import { DocsSection } from "../../../ui/vendor/getto-application/docs/data"
import { docsDescription, docsSection } from "../../../ui/vendor/getto-application/docs/helper"

export const docs_auth_ticket: DocsSection[] = [
    docsSection("認証チケット", [
        docsDescription([
            {
                title: "認証チケット",
                body: ["有効期限付きのチケット", "認証・認可のため発行するトークン"],
                help: ["サーバーアクセスに必要", "適切な方法で署名", "セキュアな方法で送信"],
            },
        ]),
        docsDescription([
            {
                title: "認証情報",
                body: ["nonce・認証時刻"],
                help: [
                    "public な手続きのため認証 nonce を送信",
                    "認証時刻を参照して必要な時に更新する",
                ],
            },
        ]),
        docsDescription([
            {
                title: "認可情報",
                body: ["nonce・権限"],
                help: [
                    "secure な手続きのため認可 nonce を送信",
                    "権限によってメニューの表示を切り替え",
                ],
            },
        ]),
    ]),
    docsSection("認証チケット有効期限", [
        docsDescription([
            {
                title: "認証トークン有効期限",
                body: ["認証チケット更新の期限"],
                help: ["認証方法によらず一定の期間が設定される"],
            },
        ]),
        docsDescription([
            {
                title: "認可トークン有効期限",
                body: ["APIサーバーアクセスの期限"],
                help: ["認証方法によらず一定の期間が設定される"],
            },
        ]),
        docsDescription([
            {
                title: "コンテンツトークン有効期限",
                body: ["コンテンツサーバーアクセスの期限"],
                help: ["認証方法によらず一定の期間が設定される"],
            },
        ]),
        docsDescription([
            {
                title: "最大延長期限",
                body: ["有効期限更新リクエストの期限"],
                help: ["パスワード認証なら短め", "web 証明書認証なら長めを想定"],
            },
        ]),
    ]),
    docsSection("認証チケットデータ", [
        docsDescription([
            {
                title: "保存するデータ",
                body: ["ログインID", "有効期限", "最大延長期限"],
                help: ["有効期限が切れたものは無効として扱う"],
            },
        ]),
    ]),
    docsSection("コンテンツサーバー", [
        docsDescription([
            {
                title: "public",
                body: ["認証情報の不要なコンテンツ"],
                help: ["認証とアップデートを行うための入り口"],
            },
            {
                title: "secure",
                body: ["認証情報の必要なコンテンツ"],
                help: ["コンテンツトークン送信でアクセス可能", "アプリケーション本体"],
            },
        ]),
    ]),
    docsSection("APIサーバー", [
        docsDescription([
            {
                title: "public",
                body: ["認証情報に関する手続きを行う"],
                help: [
                    "認証トークン・nonce 送信でアクセス可能",
                    "認証情報のチェック・更新・破棄など",
                ],
            },
            {
                title: "secure",
                body: ["認証が必要な手続きを行う"],
                help: [
                    "認可トークン・nonce 送信でアクセス可能",
                    "アプリケーション本体の手続き",
                    "ログインユーザーの認証プロフィール変更",
                    "認証ユーザーの管理",
                ],
            },
        ]),
    ]),
]
