import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { box } from "../../../../../../ui/vendor/getto-css/preact/design/box"
import { button_send, field } from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../../../ui/vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../../../ui/vendor/getto-css/preact/design/alignment"

import { LogoutResource, LogoutResourceState } from "../resource"

import { RepositoryError } from "../../../../../z_details/_ui/repository/data"
import { ClearAuthTicketError } from "../../clear/data"
import { remoteCommonError } from "../../../../../z_details/_ui/remote/helper"

export function LogoutEntry(resource: LogoutResource): VNode {
    return h(LogoutComponent, {
        ...resource,
        state: useApplicationAction(resource.logout),
    })
}

type Props = LogoutResource & LogoutResourceState
export function LogoutComponent(props: Props): VNode {
    useLayoutEffect(() => {
        switch (props.state.type) {
            case "succeed-to-logout":
                // credential が削除されているので、reload するとログイン画面になる
                location.reload()
                break
        }
    }, [props.state])

    switch (props.state.type) {
        case "initial-logout":
        case "succeed-to-logout":
            return logoutBox({ success: true })

        case "failed-to-logout":
            return logoutBox({ success: false, err: { type: "repository", err: props.state.err } })

        case "failed-to-clear":
            return logoutBox({ success: false, err: { type: "remote", err: props.state.err } })
    }

    type LogoutBoxContent =
        | Readonly<{ success: true }>
        | Readonly<{ success: false; err: LogoutBoxError }>
    type LogoutBoxError =
        | Readonly<{ type: "repository"; err: RepositoryError }>
        | Readonly<{ type: "remote"; err: ClearAuthTicketError }>

    function logoutBox(content: LogoutBoxContent): VNode {
        return box({
            body: [
                v_small(),
                field({
                    title: "ログアウト",
                    body: button_send({ label: "ログアウト", state: "normal", onClick }),
                    help: ["作業完了後ログアウトしてください"],
                }),
                ...error(),
            ],
        })

        function onClick() {
            props.logout.submit()
        }

        function error(): VNode[] {
            if (content.success) {
                return []
            }
            return [
                v_small(),
                notice_alert("ログアウトの処理中にエラーが発生しました"),
                ...detail(content.err),
            ]

            function detail(err: LogoutBoxError): VNode[] {
                switch (err.type) {
                    case "repository":
                        return errorDetail(err.err.err).map(p)

                    case "remote":
                        return remoteError(err.err).map(p)
                }

                function p(message: string): VNode {
                    return html`<p>${message}</p>`
                }
            }
            function remoteError(err: ClearAuthTicketError): string[] {
                return remoteCommonError(err, (reason) => [
                    `${reason.message}によりログアウトに失敗しました`,
                    ...reason.detail,
                ])
            }
            function errorDetail(err: string): string[] {
                if (err.length === 0) {
                    return []
                }
                return [`詳細: ${err}`]
            }
        }
    }
}
