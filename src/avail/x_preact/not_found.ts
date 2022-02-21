import { VNode } from "preact"
import { html } from "htm/preact"

import { env } from "../../y_environment/ui/env"

import { loginBox } from "../../z_vendor/getto-css/preact/layout/login"
import { buttons } from "../../z_vendor/getto-css/preact/design/form"
import { lnir } from "../../z_lib/ui/icon/line_icon"

import { useDocumentTitle } from "../../core/x_preact/hooks"
import { siteInfo } from "../../x_content/site"
import { icon } from "../../core/x_preact/design/icon"

const pageTitle = "Not Found" as const

type Props = {
    // no props
}
export function NotFoundComponent(_props: Props): VNode {
    useDocumentTitle(pageTitle)

    return loginBox(siteInfo, {
        title: "リンクが切れていました",
        body: [
            html`<p>
                リンクされたページが見つかりませんでした<br />
                これはシステム側の不備です
            </p>`,
            html`<p>
                お手数ですが、管理者にクリックしたリンクをお伝えください<br />
                直前まで行っていた作業も教えていただけると助かります
            </p>`,
            html`<p>作業は左下のリンクからホームに戻って続けられます</p>`,
        ],
        footer: buttons({
            left: [html`<a href="${homeHref()}">${icon(lnir(["home"]))} ホームへ</a>`],
        }),
    })

    function homeHref() {
        return `/${env.version}/index.html`
    }
}
