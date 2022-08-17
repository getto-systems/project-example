import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidationMessage } from "../../../../../common/x_preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { LoginIdField } from "../../../login_id/input/x_preact/field"
import { AuthUserMemoField, AuthUserGrantedRolesField } from "../../input/field/x_preact/input"
import { ResetTokenDestinationField } from "../../../password/reset/token_destination/input/x_preact/input"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { RegisterButton } from "../../../../../common/x_preact/button/register_button"
import { RegisterSuccessButton } from "../../../../../common/x_preact/button/register_success_button"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { RegisterAuthUserAccountAction } from "../action"

import { RegisterAuthUserAccountError } from "../data"

type Props = Readonly<{
    register: RegisterAuthUserAccountAction
}>
export function RegisterAuthUserAccount(props: Props): VNode {
    return container(
        box({
            form: true,
            title: "新規ユーザー登録",
            body: [
                h(LoginIdField, { field: props.register.loginId }),
                h(AuthUserMemoField, { field: props.register.memo }),
                h(AuthUserGrantedRolesField, { field: props.register.grantedRoles }),
                h(ResetTokenDestinationField, { field: props.register.resetTokenDestination }),
            ],
            footer: h(Footer, {}),
        }),
    )

    function Footer(_props: unknown): VNode {
        return html`${[
            buttons({ left: h(Submit, {}), right: h(Clear, {}) }),
            h(ValidationMessage, props.register.validate),
            h(Message, {}),
        ]}`
    }

    function Submit(_props: unknown): VNode {
        const registerState = useApplicationState(props.register.state)
        const validateState = useApplicationState(props.register.validate.state)
        const observeState = useApplicationState(props.register.observe.state)

        if (registerState.type === "success") {
            return h(RegisterSuccessButton, { onClick })
        } else {
            return h(RegisterButton, {
                isConnecting: registerState.type === "try",
                validateState,
                observeState,
                onClick,
            })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.register.submit()
        }
    }

    function Clear(_props: unknown): VNode {
        const observeState = useApplicationState(props.register.observe.state)

        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.register.clear()
        }
    }

    function Message(_props: unknown): VNode {
        const registerState = useApplicationState(props.register.state)

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

function modifyError(err: RegisterAuthUserAccountError): readonly VNodeContent[] {
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
