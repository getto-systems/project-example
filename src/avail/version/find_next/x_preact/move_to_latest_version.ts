import { h, VNode } from "preact"
import { useErrorBoundary, useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../z_vendor/getto-css/preact/layout/login"
import { siteInfo } from "../../../../x_content/site"
import { spinner } from "../../../../core/x_preact/design/icon"

import { ApplicationErrorComponent } from "../../../x_preact/application_error"

import { applicationPath } from "../../find_next/helper"

import { CheckDeployExistsError } from "../../find_next/data"
import { ApplicationView } from "../../../../z_vendor/getto-application/action/action"
import { FindNextVersionAction, FindNextVersionState } from "../action"

export function MoveToLatestVersionEntry(view: ApplicationView<FindNextVersionAction>): VNode {
    const action = useApplicationView(view)
    const state = useApplicationAction(action)
    const [err] = useErrorBoundary((err) => {
        // 認証前なのでエラーはどうしようもない
        console.log(err)
    })

    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(MoveToLatestVersionComponent, { findNext: action, state })
}

type Props = Readonly<{
    findNext: FindNextVersionAction
    state: FindNextVersionState
}>
export function MoveToLatestVersionComponent(props: Props): VNode {
    useLayoutEffect(() => {
        switch (props.state.type) {
            case "succeed-to-find":
                // /index.html から呼び出されるので、最新かによらず
                // /${version}/index.html に遷移する
                location.href = applicationPath(props.state.version, props.state.target)
                break
        }
    }, [props.state])

    switch (props.state.type) {
        case "initial-next-version":
            return EMPTY_CONTENT

        case "take-longtime-to-find":
            return takeLongtimeMessage()

        case "succeed-to-find":
            // location の変更は useLayoutEffect で行うので中身は空
            return EMPTY_CONTENT

        case "failed-to-find":
            return h(ApplicationErrorComponent, { err: errorMessage(props.state.err) })
    }

    function takeLongtimeMessage() {
        return loginBox(siteInfo, {
            title: "アプリケーション読み込み中",
            body: [
                html`<p>${spinner} アプリケーションの読み込みに時間がかかっています</p>`,
                html`<p>
                    30秒以上かかるようであれば何かがおかしいので、<br />
                    お手数ですが、管理者にお伝えください
                </p>`,
            ],
            footer: "",
        })
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

const EMPTY_CONTENT = html``
