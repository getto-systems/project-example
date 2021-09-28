import { VNode } from "preact"
import { html } from "htm/preact"

import { v_small } from "../../../ui/vendor/getto-css/preact/design/alignment"
import { buttons, field } from "../../../ui/vendor/getto-css/preact/design/form"
import { loginBox } from "../../../ui/vendor/getto-css/preact/layout/login"

import { env } from "../../y_environment/_ui/env"

import { siteInfo } from "../../example/site"

type Props = Readonly<{
    err: string
}>
export function ApplicationErrorComponent(props: Props): VNode {
    return loginBox(siteInfo, {
        title: html`システムエラーが発生しました`,
        body: [
            html`<p>
                エラーが発生したため、処理を中断しました<br />
                これはシステム側の不備です
            </p>`,
            v_small(),
            field({ title: "画面", body: html`<pre>${location.pathname}</pre>`, help: [location.host] }),
            field({ title: "詳細", body: detail(props.err) }),
            html`<p>
                お手数ですが、管理者に詳細をお伝えください<br />
                直前まで行っていた作業も教えていただけると助かります
            </p>`,
            html`<p>
                左下のリンクで再読み込みすることで解消するかもしれません<br />
                繰り返しエラーになる場合は右下のホームから戻ってください
            </p>`,
        ],
        footer: buttons({ left: [reloadLink()], right: [topLink()] }),
    })

    function detail(err: string) {
        if (err.length === 0) {
            return "（詳細な内容は取得できませんでした）"
        }
        return err
    }

    function topLink() {
        return html`<a href="/${env.version}/index.html"><i class="lnir lnir-home"></i> ホーム</a>`
    }
    function reloadLink() {
        // search param をリセットしてやり直してみる
        return html`<a href="?"><i class="lnir lnir-reload"></i> 再読み込み</a>`
    }
}
