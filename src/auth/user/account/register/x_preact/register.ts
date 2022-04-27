import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, validationMessage } from "../../../../../core/x_preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { LoginIdField } from "../../../login_id/input/x_preact/input"
import { AuthUserMemoField } from "../../input/memo/x_preact/input"
import { GrantedRolesField } from "../../input/granted_roles/x_preact/input"
import { ResetTokenDestinationField } from "../../../password/reset/token_destination/input/x_preact/input"
import { ClearChangesButton } from "../../../../../core/x_preact/button/clear_changes_button"
import { RegisterButton } from "../../../../../core/x_preact/button/register_button"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { RegisterAuthUserAccountAction } from "../action"

import { RegisterAuthUserAccountError } from "../data"

type Props = Readonly<{
    register: RegisterAuthUserAccountAction
}>
export function RegisterAuthUserAccount(props: Props): VNode {
    const state = useApplicationAction(props.register)
    const validateState = useApplicationAction(props.register.validate)
    const observeState = useApplicationAction(props.register.observe)

    return container(
        box({
            form: true,
            title: "新規ユーザー登録",
            body: [
                h(LoginIdField, { field: props.register.loginId }),
                h(AuthUserMemoField, { field: props.register.memo }),
                h(GrantedRolesField, { field: props.register.grantedRoles }),
                h(ResetTokenDestinationField, { field: props.register.resetTokenDestination }),
            ],
            footer: [
                buttons({
                    left: submitButton(),
                    right: clearButton(),
                }),
                ...validationMessage(validateState),
                ...message(),
            ],
        }),
    )

    function submitButton(): VNode {
        return h(RegisterButton, {
            isSuccess: state.type === "success",
            isConnecting: state.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.register.submit()
        }
    }

    function clearButton(): VNode {
        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.register.clear()
        }
    }

    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
                return []

            case "try":
                if (state.hasTakenLongtime) {
                    return [takeLongtimeField("変更")]
                }
                return []

            case "failed":
                return [fieldHelp_error(modifyError(state.err))]
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
