import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidateBoardMessage } from "../../../../../common/x_preact/design/form"

import { AuthUserLoginIdField } from "../../../login_id/input/field/x_preact/input"
import { AuthPermissionGrantedField } from "../../../kernel/input/field/x_preact/input"
import { AuthUserMemoField } from "../../input/field/x_preact/input"
import { ResetTokenDestinationField } from "../../../password/reset/token_destination/input/field/x_preact/input"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { RegisterButton } from "../../../../../common/x_preact/button/register_button"

import { remoteCommonErrorReason } from "../../../../../common/util/remote/x_error/reason"

import { RegisterAuthUserAccountAction } from "../action"

import { RegisterAuthUserAccountError } from "../data"

type Props = Readonly<{
    register: RegisterAuthUserAccountAction
}>
export function RegisterAuthUserAccount(props: Props): PreactNode {
    return container(
        box({
            form: true,
            title: "新規ユーザー登録",
            body: [
                h(AuthUserLoginIdField, { field: props.register.loginId }),
                h(AuthUserMemoField, { field: props.register.memo }),
                h(AuthPermissionGrantedField, { field: props.register.granted }),
                h(ResetTokenDestinationField, { field: props.register.resetTokenDestination }),
            ],
            footer: h(Footer, {}),
        }),
    )

    function Footer(_props: unknown): PreactNode {
        return html`${[
            buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
            h(ValidateBoardMessage, { state: props.register.validate }),
            h(Message, {}),
        ]}`
    }

    function Submit(_props: unknown): PreactNode {
        return h(RegisterButton, {
            success: props.register.success,
            connect: props.register.connect,
            validate: props.register.validate,
            observe: props.register.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.register.submit()
        }
    }

    function Reset(_props: unknown): PreactNode {
        return h(ClearChangesButton, {
            observe: props.register.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.register.reset()
        }
    }

    function Message(_props: unknown): PreactNode {
        const registerState = useAtom(props.register.state)

        switch (registerState.type) {
            case "initial":
            case "success":
                return html``

            case "try":
                if (registerState.hasTakenLongtime) {
                    return takeLongtimeField("変更")
                }
                return html``

            case "failed":
                return fieldHelp_error(modifyError(registerState.err))
        }
    }
}

function modifyError(err: RegisterAuthUserAccountError): readonly PreactContent[] {
    switch (err.type) {
        case "login-id-already-registered":
            return ["指定したログインIDはすでに登録されています"]

        case "invalid":
            return ["データが正しくありません", "一旦リロードしてやり直してください"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により変更に失敗しました`,
                ...reason.detail,
            ])
    }
}
