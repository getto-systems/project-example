import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../z_details/_ui/remote/reason"
import { repositoryErrorReason } from "../../../../z_details/_ui/repository/reason"

import { useApplicationAction } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { box } from "../../../../../ui/vendor/getto-css/preact/design/box"
import { button_send, field } from "../../../../../ui/vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../../ui/vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../../ui/vendor/getto-css/preact/design/alignment"

import { LogoutResource, LogoutResourceState } from "../resource"

import { RepositoryError } from "../../../../z_details/_ui/repository/data"
import { LogoutError } from "../../logout/data"

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

    return basedOn(props)

    function basedOn({ state }: LogoutResourceState): VNode {
        switch (state.type) {
            case "succeed-to-logout":
                // reload するので何も描画しない
                return EMPTY_CONTENT

            case "initial-logout":
                return logoutBox({ initial: true })

            case "repository-error":
                return logoutBox({ initial: false, err: repositoryError(state.err) })

            case "failed-to-logout":
                return logoutBox({ initial: false, err: logoutError(state.err) })
        }
        function repositoryError(err: RepositoryError): string[] {
            return repositoryErrorReason(err, (reason) => [
                `${reason.message}によりログアウトに失敗しました`,
                ...reason.detail,
            ])
        }
        function logoutError(err: LogoutError): string[] {
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりログアウトに失敗しました`,
                ...reason.detail,
            ])
        }
    }

    type LogoutBoxContent =
        | Readonly<{ initial: true }>
        | Readonly<{ initial: false; err: string[] }>
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

        function error(): VNode[] {
            if (content.initial) {
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

const EMPTY_CONTENT = html``
