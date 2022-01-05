import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../z_lib/ui/remote/reason"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../../ui/vendor/getto-css/preact/layout/login"
import { v_medium } from "../../../../../ui/vendor/getto-css/preact/design/alignment"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"
import { siteInfo } from "../../../../example/site"
import { spinner } from "../../../../example/x_preact/design/icon"

import { appendScript } from "../../../sign/x_preact/script"

import { ApplicationErrorComponent } from "../../../../avail/x_preact/application_error"

import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { CheckAuthTicketAction, CheckAuthTicketState } from "../action"

import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"

export function CheckAuthTicketEntry(view: ApplicationView<CheckAuthTicketAction>): VNode {
    const check = useApplicationView(view)
    return h(CheckAuthTicketComponent, {
        check,
        state: useApplicationAction(check),
    })
}

type Props = Readonly<{
    check: CheckAuthTicketAction
    state: CheckAuthTicketState
}>
export function CheckAuthTicketComponent(props: Props): VNode {
    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (props.state.type) {
            case "try-to-instant-load":
                if (!props.state.scriptPath.valid) {
                    props.check.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${props.state.type}`,
                    })
                    break
                }
                appendScript(props.state.scriptPath.value, (script) => {
                    script.onload = () => {
                        props.check.succeedToInstantLoad()
                    }
                    script.onerror = () => {
                        props.check.failedToInstantLoad()
                    }
                })
                break

            case "try-to-load":
                if (!props.state.scriptPath.valid) {
                    props.check.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${props.state.type}`,
                    })
                    break
                }
                appendScript(props.state.scriptPath.value, (script) => {
                    script.onerror = () => {
                        props.check.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${props.state.type}`,
                        })
                    }
                })
                break
        }
    }, [props.check, props.state])

    switch (props.state.type) {
        case "initial-check":
        case "required-to-login":
            return EMPTY_CONTENT

        case "try-to-instant-load":
        case "try-to-load":
            // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
            return EMPTY_CONTENT

        case "succeed-to-start-continuous-renew":
        case "succeed-to-renew":
        case "ticket-not-expired":
            // これらはスクリプトがロードされた後に発行される
            // したがって、un-mount されているのでここには来ない
            return EMPTY_CONTENT

        case "try-to-renew":
            // すぐに帰ってくることを想定
            return EMPTY_CONTENT

        case "take-longtime-to-renew":
            return takeLongtimeMessage()

        case "failed-to-renew":
            return errorMessage(props.state.err)

        case "repository-error":
        case "load-error":
            return h(ApplicationErrorComponent, { err: props.state.err.err })
    }

    function takeLongtimeMessage() {
        return loginBox(siteInfo, {
            title: "認証に時間がかかっています",
            body: [
                html`<p>${spinner} 認証処理中です</p>`,
                html`<p>
                    30秒以上かかる場合は何かがおかしいので、
                    <br />
                    お手数ですが管理者に連絡お願いします
                </p>`,
            ],
        })
    }
    function errorMessage(err: RemoteCommonError): VNode {
        return loginBox(siteInfo, {
            title: "認証に失敗しました",
            body: [
                ...renewError(err).map((message) => html`<p>${message}</p>`),
                v_medium(),
                html`<p>お手数ですが、上記メッセージを管理者にお伝えください</p>`,
            ],
        })
    }
}

function renewError(err: RemoteCommonError): VNodeContent[] {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}により認証に失敗しました`,
        ...reason.detail,
    ])
}

const EMPTY_CONTENT: VNode = html``
