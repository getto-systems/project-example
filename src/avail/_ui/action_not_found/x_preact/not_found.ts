import { h, VNode } from "preact"
import { useErrorBoundary } from "preact/hooks"
import { html } from "htm/preact"

import { useApplicationView } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../../ui/vendor/getto-css/preact/layout/login"
import { buttons } from "../../../../../ui/vendor/getto-css/preact/design/form"

import { useDocumentTitle } from "../../../../example/_ui/x_preact/hooks"
import { siteInfo } from "../../../../example/site"
import { icon } from "../../../../example/_ui/x_preact/design/icon"

import { ApplicationErrorComponent } from "../../x_preact/application_error"

import { NotFoundView, NotFoundResource } from "../resource"

export function NotFoundEntry(view: NotFoundView): VNode {
    const resource = useApplicationView(view)

    const [err] = useErrorBoundary((err) => {
        // 認証していないのでエラーはどうしようもない
        console.log(err)
    })
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(NotFoundComponent, resource)
}

const pageTitle = "Not Found" as const

type Props = NotFoundResource
export function NotFoundComponent(props: Props): VNode {
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
            left: [html`<a href="${homeHref()}">${icon("home")} ホームへ</a>`],
        }),
    })

    function homeHref() {
        return `/${props.version.getCurrent()}/index.html`
    }
}
