import { html } from "htm/preact"
import { PreactNode } from "../../common/x_preact/node"

import { v_small } from "../../z_vendor/getto-css/preact/design/alignment"
import { buttons, field } from "../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../z_vendor/getto-css/preact/layout/login"

import { lnir } from "../../common/util/icon/detail/line_icon"

import { env } from "../../y_environment/ui/env"

import { iconHtml } from "../../common/util/icon/x_preact/icon"
import { icon_home } from "../../x_content/icon"
import { siteInfo } from "../../x_content/site"

export function ApplicationError(
    props: Readonly<{
        err: string
    }>,
): PreactNode {
    return loginBox(siteInfo, {
        title: html`システムエラーが発生しました`,
        body: [
            html`<p>
                エラーが発生したため、処理を中断しました<br />
                これはシステム側の不備です<br />
                お手数ですが、管理者に詳細をお伝えください
            </p>`,
            v_small(),
            field({
                title: "画面",
                body: html`<pre>${location.pathname}</pre>`,
                help: [location.host],
            }),
            field({ title: "詳細", body: detail(props.err) }),
            html`<p>直前まで行っていた作業も教えていただけると助かります</p>`,
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
        return html`<a href="/${env.version}/index.html">${iconHtml(icon_home)} ホーム</a>`
    }
    function reloadLink() {
        // search param をリセットしてやり直してみる
        return html`<a href="?">${iconHtml(lnir(["reload"]))} 再読み込み</a>`
    }
}
