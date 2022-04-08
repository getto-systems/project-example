import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../z_lib/ui/remote/x_error/reason"
import { repositoryErrorReason } from "../../../../z_lib/ui/repository/x_error/reason"

import { useApplicationAction } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { box } from "../../../../z_vendor/getto-css/preact/design/box"
import { button_send, field } from "../../../../z_vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../z_vendor/getto-css/preact/design/alignment"

import { LogoutAction, LogoutState } from "../action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"

type Resource = Readonly<{
    logout: LogoutAction
}>
export function LogoutEntry(resource: Resource): VNode {
    return h(LogoutComponent, <Props>{
        ...resource,
        state: useApplicationAction(resource.logout),
    })
}

type Props = Resource & Readonly<{ state: LogoutState }>
export function LogoutComponent(props: Props): VNode {
    useLayoutEffect(() => {
        switch (props.state.type) {
            case "success":
                // credential が削除されているので、reload するとログイン画面になる
                location.reload()
                break
        }
    }, [props.state])

    return basedOn(props)

    function basedOn({ state }: Readonly<{ state: LogoutState }>): VNode {
        switch (state.type) {
            case "initial":
            case "success":
                return logoutBox({ type: "initial" })

            case "repository-error":
                return logoutBox({ type: "error", err: repositoryError(state.err) })

            case "failed":
                return logoutBox({ type: "error", err: logoutError(state.err) })
        }
    }

    type LogoutBoxContent =
        | Readonly<{ type: "initial" }>
        | Readonly<{ type: "error"; err: readonly string[] }>
    function logoutBox(content: LogoutBoxContent): VNode {
        return box({
            body: [
                v_small(),
                field({
                    title: "ログアウト",
                    body: logoutButton(),
                    help: ["作業完了後ログアウトしてください"],
                }),
                ...error(),
            ],
        })

        function logoutButton() {
            return button_send({ label: "ログアウト", state: "normal", onClick })

            function onClick() {
                props.logout.submit()
            }
        }

        function error(): readonly VNode[] {
            if (content.type === "initial") {
                return []
            }
            return [
                v_small(),
                notice_alert("ログアウトの処理中にエラーが発生しました"),
                ...content.err.map((message) => html`<p>${message}</p>`),
            ]
        }
    }
}

function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりログアウトに失敗しました`,
        ...reason.detail,
    ])
}
function logoutError(err: RemoteCommonError): readonly string[] {
    return remoteCommonErrorReason(err, (reason) => [
        `${reason.message}によりログアウトに失敗しました`,
        ...reason.detail,
    ])
}
