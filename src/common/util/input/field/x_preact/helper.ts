import { PreactContent } from "../../../../x_preact/vnode"

import { ValidateBoardValue } from "../../../board/validate/data"

type ValidateState =
    | Readonly<{ type: "normal" }>
    | Readonly<{ type: "error"; notice: readonly PreactContent[] }>

export function mapValidateBoardValue<T, E>(
    result: ValidateBoardValue<T, E>,
    map: (err: E) => readonly PreactContent[],
): ValidateState {
    return result.valid ? { type: "normal" } : { type: "error", notice: map(result.err) }
}
