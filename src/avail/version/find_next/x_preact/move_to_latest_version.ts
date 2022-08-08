import { h, VNode } from "preact"
import { useErrorBoundary, useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../z_vendor/getto-css/preact/layout/login"
import { siteInfo } from "../../../../x_content/site"
import { icon_spinner } from "../../../../x_content/icon"
import { applicationPath } from "../../find_next/helper"
import { iconHtml } from "../../../../z_lib/ui/icon/x_preact/icon"

import { ApplicationError } from "../../../x_preact/application_error"

import { FindNextVersionAction } from "../action"

import { CheckDeployExistsError } from "../../find_next/data"

type Props = Readonly<{
    findNext: FindNextVersionAction
}>
export function MoveToLatestVersion(props: Props): VNode {
    const state = useApplicationState(props.findNext.state)
    const [err] = useErrorBoundary((err) => {
        // 認証前なのでエラーはどうしようもない
        console.log(err)
    })

    useLayoutEffect(() => {
        switch (state.type) {
            case "success":
                // /index.html から呼び出されるので、最新かによらず
                // /${version}/index.html に遷移する
                location.href = applicationPath(state.version, state.target)
                break
        }
    }, [state])

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    switch (state.type) {
        case "initial":
            return html``

        case "take-longtime":
            return loginBox(siteInfo, {
                title: "アプリケーション読み込み中",
                body: [
                    html`<p>
                        ${iconHtml(icon_spinner)} アプリケーションの読み込みに時間がかかっています
                    </p>`,
                    html`<p>
                        30秒以上かかるようであれば何かがおかしいので、<br />
                        お手数ですが、管理者にお伝えください
                    </p>`,
                ],
                footer: "",
            })

        case "success":
            // location の変更は useLayoutEffect で行うので中身は空
            return html``

        case "failed":
            return h(ApplicationError, { err: errorMessage(state.err) })
    }
}

function errorMessage(err: CheckDeployExistsError): string {
    switch (err.type) {
        case "server-error":
            return "サーバーエラーが発生しました"

        case "infra-error":
            return `ネットワークエラーが発生しました: ${err.err}`
    }
}
