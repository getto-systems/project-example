import { h } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../common/x_preact/node"

import { remoteCommonErrorReason } from "../../../../common/util/remote/x_error/reason"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import { loginBox } from "../../../../z_vendor/getto-css/preact/layout/login"
import { v_medium } from "../../../../z_vendor/getto-css/preact/design/alignment"

import { siteInfo } from "../../../../x_content/site"
import { icon_spinner } from "../../../../x_content/icon"
import { iconHtml } from "../../../../common/util/icon/x_preact/icon"

import { appendScript } from "../../../sign/x_preact/script"

import { ApplicationError } from "../../../../avail/x_preact/application_error"

import { AuthenticateWithTokenAction } from "../action"

import { RemoteCommonError } from "../../../../common/util/remote/data"

type Props = Readonly<{
    check: AuthenticateWithTokenAction
}>
export function CheckAuthTicket(props: Props): PreactNode {
    const state = useAtom(props.check.state)

    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (state.type) {
            case "try-to-instant-load":
                if (!state.scriptPath.valid) {
                    props.check.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${state.type}`,
                    })
                    break
                }
                appendScript(state.scriptPath.value, (script) => {
                    script.onload = () => {
                        props.check.succeedToInstantLoad()
                    }
                    script.onerror = () => {
                        props.check.failedToInstantLoad()
                    }
                })
                break

            case "try-to-load":
                if (!state.scriptPath.valid) {
                    props.check.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${state.type}`,
                    })
                    break
                }
                appendScript(state.scriptPath.value, (script) => {
                    script.onerror = () => {
                        props.check.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${state.type}`,
                        })
                    }
                })
                break
        }
    }, [props.check, state])

    switch (state.type) {
        case "initial-check":
        case "required-to-login":
            return html``

        case "try-to-instant-load":
        case "try-to-load":
            // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
            return html``

        case "succeed-to-start-continuous-renew":
        case "succeed-to-renew":
        case "ticket-not-expired":
            // これらはスクリプトがロードされた後に発行される
            // したがって、un-mount されているのでここには来ない
            return html``

        case "try-to-renew":
            if (state.hasTakenLongtime) {
                return loginBox(siteInfo, {
                    title: "認証に時間がかかっています",
                    body: [
                        html`<p>${iconHtml(icon_spinner)} 認証に時間がかかっています</p>`,
                        html`<p>
                            30秒以上かかる場合は何かがおかしいので、
                            <br />
                            お手数ですが管理者に連絡お願いします
                        </p>`,
                    ],
                })
            }
            // すぐに帰ってくることを想定
            return html``

        case "failed-to-renew":
            return loginBox(siteInfo, {
                title: "認証に失敗しました",
                body: [
                    ...renewError(state.err).map((message) => html`<p>${message}</p>`),
                    v_medium(),
                    html`<p>お手数ですが、上記メッセージを管理者にお伝えください</p>`,
                ],
            })

        case "repository-error":
        case "load-error":
            return h(ApplicationError, { err: state.err.err })
    }
}

function renewError(err: RemoteCommonError): readonly PreactContent[] {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により認証に失敗しました`,
        ...reason.detail,
    ])
}
