import { VNodeContent } from "../../../x_preact/common"

import { ValidateBoardFieldState } from "../../../../../z_vendor/getto-application/board/validate_field/action"

type ValidateState =
    | Readonly<{ type: "normal" }>
    | Readonly<{ type: "error"; notice: readonly VNodeContent[] }>

export function mapValidateState<T, E>(
    validateState: ValidateBoardFieldState<T, E>,
    map: (err: E) => readonly VNodeContent[],
): ValidateState {
    return validateState.type === "initial" || validateState.result.valid
        ? { type: "normal" }
        : { type: "error", notice: map(validateState.result.err) }
}
