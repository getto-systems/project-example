import { html } from "htm/preact"
import { PreactNode } from "../../common/x_preact/vnode"

import { env } from "../../y_environment/ui/env"

import { loginBox } from "../../z_vendor/getto-css/preact/layout/login"
import { buttons } from "../../z_vendor/getto-css/preact/design/form"

import { useDocumentTitle } from "../../common/x_preact/hooks"
import { siteInfo } from "../../x_content/site"
import { iconHtml } from "../../common/util/icon/x_preact/icon"
import { icon_home } from "../../x_content/icon"

export function NotFound(_props: unknown): PreactNode {
    useDocumentTitle("Not Found")

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
            left: [html`<a href="${homeHref()}">${iconHtml(icon_home)} ホームへ</a>`],
        }),
    })

    function homeHref() {
        return `/${env.version}/index.html`
    }
}
