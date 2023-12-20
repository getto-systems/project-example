import { h } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"
import { PreactNode } from "../../../../common/x_preact/vnode"

import { env } from "../../../../y_environment/ui/env"

import { remoteCommonErrorReason } from "../../../../common/util/remote/x_error/reason"
import { repositoryErrorReason } from "../../../../common/util/repository/x_error/reason"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import { box, container } from "../../../../z_vendor/getto-css/preact/design/box"
import { button_send, field } from "../../../../z_vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../z_vendor/getto-css/preact/design/alignment"

import { LogoutAction } from "../action"

import { RepositoryError } from "../../../../common/util/repository/data"
import { RemoteCommonError } from "../../../../common/util/remote/data"

type Props = Readonly<{
    logout: LogoutAction
}>
export function Logout(props: Props): PreactNode {
    useRedirectOnSuccess(props.logout)

    return container([logoutBox()])

    function logoutBox(): PreactNode {
        return box({
            form: true,
            body: [
                v_small(),
                field({
                    title: "ログアウト",
                    body: logoutButton(),
                    help: ["作業完了後ログアウトしてください"],
                }),
                h(Error, {}),
            ],
        })

        function logoutButton(): PreactNode {
            return button_send({ label: "ログアウト", state: "normal", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.logout.submit()
            }
        }
    }

    function Error(_props: unknown): PreactNode {
        const state = useAtom(props.logout.state)

        switch (state.type) {
            case "initial":
            case "success":
                return html``

            case "repository-error":
                return html`${errorMessage(repositoryError(state.err))}`

            case "failed":
                return html`${errorMessage(logoutError(state.err))}`
        }

        function errorMessage(err: readonly string[]): readonly PreactNode[] {
            return [
                v_small(),
                notice_alert("ログアウトの処理中にエラーが発生しました"),
                ...err.map((message) => html`<p>${message}</p>`),
            ]
        }
    }
}
function useRedirectOnSuccess(logout: LogoutAction): void {
    const state = useAtom(logout.state)

    useLayoutEffect(() => {
        switch (state.type) {
            case "success":
                location.href = `/${env.version}/index.html`
                break
        }
    }, [state])
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
