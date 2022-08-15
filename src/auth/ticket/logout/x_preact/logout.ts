import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { env } from "../../../../y_environment/ui/env"

import { remoteCommonErrorReason } from "../../../../z_lib/ui/remote/x_error/reason"
import { repositoryErrorReason } from "../../../../z_lib/ui/repository/x_error/reason"

import { useApplicationState } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { box, container } from "../../../../z_vendor/getto-css/preact/design/box"
import { button_send, field, form } from "../../../../z_vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../z_vendor/getto-css/preact/design/alignment"

import { LogoutAction } from "../action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"

type Props = Readonly<{
    logout: LogoutAction
}>
export function Logout(props: Props): VNode {
    useRedirectOnSuccess(props.logout)

    return form(container([logoutBox()]))

    function logoutBox(): VNode {
        return box({
            body: [
                v_small(),
                field({
                    title: "ログアウト",
                    body: logoutButton(),
                    help: ["作業完了後ログアウトしてください"],
                }),
                h(Error, props),
            ],
        })

        function logoutButton(): VNode {
            return button_send({ label: "ログアウト", state: "normal", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.logout.submit()
            }
        }
    }
}
function useRedirectOnSuccess(logout: LogoutAction): void {
    const state = useApplicationState(logout.state)

    useLayoutEffect(() => {
        switch (state.type) {
            case "success":
                location.href = `/${env.version}/index.html`
                break
        }
    }, [state])
}

function Error(props: Props): VNode {
    const state = useApplicationState(props.logout.state)

    switch (state.type) {
        case "initial":
        case "success":
            return html``

        case "repository-error":
            return html`${errorMessage(repositoryError(state.err))}`

        case "failed":
            return html`${errorMessage(logoutError(state.err))}`
    }

    function errorMessage(err: readonly string[]): readonly VNode[] {
        return [
            v_small(),
            notice_alert("ログアウトの処理中にエラーが発生しました"),
            ...err.map((message) => html`<p>${message}</p>`),
        ]
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
