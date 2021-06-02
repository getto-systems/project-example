import { VNode } from "preact"
import { html } from "htm/preact"

import { box, box_double, container } from "../../../../../../../../../ui/vendor/getto-css/preact/design/box"
import { field } from "../../../../../../../../../ui/vendor/getto-css/preact/design/form"
import { v_small } from "../../../../../../../../../ui/vendor/getto-css/preact/design/alignment"

import { items, itemsSection } from "../../../box"

export const content_development_auth_api = (): VNode[] => [
    container([credential(), ticket(), user(), password(), passwordReset(), request()]),
]

function credential() {
    return box_double({
        title: "認証情報",
        body: [
            field({
                title: "ACTION",
                body: [
                    items([
                        "チケットトークン検証",
                        "チケットトークン発行",
                        "APIトークン発行",
                        "コンテンツトークン発行",
                    ]),
                ],
            }),
            field({
                title: "チケットトークン",
                body: [
                    html`
                        <p>API トークン・コンテンツトークンの有効期限延長に使用するチケット</p>
                        <p>適切な方法で署名する</p>
                        <p>適切な方法で署名を検証する</p>
                    `,
                    v_small(),
                    html`
                        <p>チケットトークンの有効期限は1週間程度</p>
                        <p>（金曜の業務終了から月曜の使用開始までに有効期限が切れないように）</p>
                    `,
                    v_small(),
                    html`
                        <p>チケットトークンをクライアントに送信する際は特にセキュアな方法を使用</p>
                        <p>（secure, http only な cookie を想定）</p>
                    `,
                    v_small(),
                    itemsSection("トークンには以下のデータを含める", ["Nonce", "ユーザー", "有効期限"]),
                    html`
                        <p>Nonce は別途クライアントに送信し、認証時に再送させる</p>
                        <p>Nonce がトークンのものと異なる場合は認証失敗とし、認証情報を失効させる</p>
                    `,
                ],
            }),
            field({
                title: "APIトークン",
                body: [
                    html`
                        <p>API サーバーを使用するために必要な認証情報</p>
                        <p>適切な方法で署名する</p>
                    `,
                    v_small(),
                    html`
                        <p>署名の検証は API サーバーで行う</p>
                        <p>（検証に必要な鍵情報は API サーバーに提供する）</p>
                    `,
                    v_small(),
                    html`
                        <p>API トークンの有効期限は5分程度とする</p>
                        <p>（API トークンが漏れたときの被害を小さくする）</p>
                    `,
                ],
            }),
            field({
                title: "コンテンツトークン",
                body: [
                    html`
                        <p>プライベートコンテンツにアクセスするために必要な認証情報</p>
                        <p>適切な方法で署名する</p>
                        <p>署名の検証はコンテンツサーバーで行う</p>
                        <p>コンテンツトークンの有効期限は API トークンと同じ</p>
                    `,
                ],
            }),
            field({
                title: "チケット最大延長期間",
                body: [
                    html`<p>リクエスト時刻から既定の時間だけ延長が可能</p>`,
                    v_small(),
                    html`
                        <p>認証の方法によって、異なる延長期間を設定</p>
                        <p>（パスワードなら短め、Web 証明書認証なら長めを想定）</p>
                    `,
                ],
            }),
            field({
                title: "チケット有効期限",
                body: [
                    html`
                        <p>リクエスト時刻から既定の時間だけ有効</p>
                        <p>認証の方法によらず、有効期限は一定</p>
                    `,
                ],
            }),
            field({
                title: "トークン有効期限",
                body: [
                    html`
                        <p>リクエスト時刻から既定の時間だけ有効</p>
                        <p>認証の方法によらず、有効期限は一定</p>
                        <p>
                            APIトークン・コンテンツトークンはそれぞれのサーバーに指定される方法で送信するので、漏洩した際の影響を小さくするため短い有効期限を設定する
                        </p>
                    `,
                ],
            }),
        ],
    })
}

function ticket() {
    return box_double({
        title: "チケット",
        body: [
            field({
                title: "ACTION",
                body: [
                    items(["チケット登録", "チケット検証", "チケット有効期限延長", "チケット無効化"]),
                ],
            }),
            field({
                title: "チケットデータ保管",
                body: [
                    itemsSection("以下のデータをデータベースに保管", [
                        "ユーザー",
                        "有効期限",
                        "最大延長期間",
                    ]),
                    html`
                        <p>ユーザーについてチケットトークンとデータベースが一致することを検証</p>
                        <p>有効期限はデータベースに保管されたものを使用して検証</p>
                        <p>検証が失敗した場合は認証情報を失効させる</p>
                        <p>チケットの無効化はデータベースの有効期限・最大延長期間を無効化</p>
                    `,
                ],
            }),
        ],
    })
}

function user() {
    return box_double({
        title: "ユーザー",
        body: [
            field({ title: "ACTION", body: [items(["ログインID取得", "ユーザーID取得"])] }),
            field({ title: "ユーザー", body: [items(["ユーザーID", "ログインID"])] }),
            field({
                title: "ユーザーID",
                body: [
                    html`
                        <p>機械的にユーザーを識別する</p>
                        <p>全ユーザーの間で一意</p>
                        <p>予測不可能な文字列</p>
                        <p>登録時に生成して割り当て</p>
                        <p>変更されない</p>
                    `,
                ],
            }),
            field({
                title: "ログインID",
                body: [
                    html`
                        <p>ログインするのに使用する</p>
                        <p>全ユーザーの間で一意</p>
                        <p>ユーザーの都合で変更可能</p>
                    `,
                ],
            }),
            field({
                title: "ユーザーID取得",
                body: [
                    html`
                        <p>ログインIDからユーザーIDを取得</p>
                        <p>登録されていない場合は取得失敗</p>
                    `,
                ],
            }),
            field({
                title: "ログインID取得",
                body: [
                    html`
                        <p>チケットトークンのユーザーからログインIDを取得</p>
                        <p>基本取得に失敗することはない</p>
                    `,
                ],
            }),
        ],
    })
}

function password() {
    return box_double({
        title: "パスワード",
        body: [
            field({ title: "ACTION", body: [items(["パスワード検証", "パスワード変更"])] }),
            field({
                title: "入力パスワード",
                body: [
                    html`
                        <p>ユーザーが入力したパスワード</p>
                        <p>メモリ以外に保管してはならない</p>
                        <p>空のパスワードは無効</p>
                        <p>長いパスワードは無効</p>
                        <p>（bcrypt を想定しているので、72バイト以上は無効）</p>
                    `,
                ],
            }),
            field({
                title: "ハッシュ化パスワード",
                body: [
                    html`
                        <p>ユーザーが入力したパスワードにハッシュ関数を適用した文字列</p>
                        <p>データベースに保管する</p>
                    `,
                ],
            }),
            field({
                title: "パスワード検証",
                body: [
                    html`
                        <p>入力パスワードが一致するかはハッシュ関数を使用して検証</p>
                        <p>（ハッシュ関数を適用して文字列一致するのではない）</p>
                    `,
                ],
            }),
            field({
                title: "パスワード変更",
                body: [
                    html`
                        <p>新しいパスワードに変更</p>
                        <p>古いパスワードは使用不可能になる</p>
                    `,
                ],
            }),
        ],
    })
}

function passwordReset() {
    return box_double({
        title: "パスワードリセット",
        body: [
            field({
                title: "ACTION",
                body: [
                    items([
                        "セッション生成",
                        "トークン送信 Job 生成",
                        "トークン送信",
                        "ステータス取得",
                        "トークン検証",
                        "セッション完了",
                    ]),
                ],
            }),
            field({
                title: "リセットセッション",
                body: [
                    html` <p>パスワードがわからなくなったときにリセットを可能にする</p> `,
                    v_small(),
                    html`
                        <p>有効期限は1時間程度を想定</p>
                        <p>（メッセージクライアントに届くまで時間がかかる可能性がある）</p>
                    `,
                    v_small(),
                    html`
                        <p>パスワードのリセットが終わったらセッションは完了</p>
                        <p>完了したセッションではパスワードはリセットできない</p>
                    `,
                ],
            }),
            field({
                title: "セッションID",
                body: [
                    html`
                        <p>セッションを識別する</p>
                        <p>全セッションの間で一意</p>
                        <p>予測不可能な文字列</p>
                        <p>生成時に生成して割り当て</p>
                        <p>変更されない</p>
                        <p>セッションを開始したクライアントに返信</p>
                    `,
                ],
            }),
            field({
                title: "トークン",
                body: [
                    html`
                        <p>パスワードリセットの検証に使用</p>
                        <p>全セッションの間で一意</p>
                        <p>予測不可能な文字列</p>
                        <p>生成時に生成して割り当て</p>
                        <p>変更されない</p>
                    `,
                    v_small(),
                    html`
                        <p>あらかじめユーザーに割り当てられたメッセージクライアントに送信</p>
                        <p>（セッションを開始したクライアントには直接送信しない）</p>
                    `,
                ],
            }),
            field({
                title: "有効期限",
                body: [
                    html`
                        <p>リクエスト時刻から既定の時間だけ有効</p>
                        <p>宛先によらず、有効期限は一定</p>
                    `,
                ],
            }),
            field({
                title: "セッションデータ",
                body: [items(["ユーザー", "リセット時ログインID", "リクエスト時刻", "有効期限"])],
            }),
            field({
                title: "宛先",
                body: [items(["ログメッセージに出力", "Slack Channel に送信", "メールアドレスに送信"])],
            }),
            field({
                title: "ステータス",
                body: [
                    items([
                        "待機中 : 作成時刻",
                        "送信中 : 処理開始時刻",
                        "完了 : 完了時刻",
                        "失敗 : 失敗時刻・理由",
                    ]),
                ],
            }),
        ],
    })
}

function request() {
    return box({
        title: "リクエスト",
        body: [
            field({
                title: "リクエスト情報",
                body: [
                    items(["リクエスト時刻", "経路情報（リモートIP）"]),
                    v_small(),
                    html`
                        <p>アプリケーションログに記録する</p>
                        <p>http クライアントを想定している</p>
                    `,
                ],
            }),
        ],
    })
}
