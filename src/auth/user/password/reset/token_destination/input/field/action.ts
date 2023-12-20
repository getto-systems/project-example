import { resetTokenDestinationEmailConverter } from "./convert"
import { restoreResetTokenDestinationEmail } from "../../kernel/convert"

import { Atom, combineAtom, composeAtom } from "../../../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../../../common/util/load/data"
import {
    SelectFieldBoard,
    TextFieldBoard,
    initSelectFieldBoard,
    initTextFieldBoard,
} from "../../../../../../../common/util/board/field/action"
import { ObserveBoardState } from "../../../../../../../common/util/board/observe/action"
import { BoardInitializer } from "../../../../../../../common/util/board/input/action"

import { ValidateTextError } from "../../../../../../../common/util/validate/data"
import {
    ResetTokenDestination,
    ResetTokenDestinationEmail,
    ResetTokenDestinationType,
} from "../../kernel/data"
import { ValidateResetTokenDestinationError } from "./data"
import { ValidateResetTokenDestinationValue } from "./data"
import { ValidateBoardValue } from "../../../../../../../common/util/board/validate/data"

export type ResetTokenDestinationField = Readonly<{
    type: ResetTokenDestinationTypeField
    email: ResetTokenDestinationEmailField
    validate: Atom<ValidateBoardValue<ResetTokenDestination, ValidateResetTokenDestinationError>>
    observe: Atom<ObserveBoardState>
}>

export type ResetTokenDestinationTypeField = SelectFieldBoard<ResetTokenDestinationType>
export type ResetTokenDestinationEmailField = TextFieldBoard<
    ResetTokenDestinationEmail,
    readonly ValidateTextError[]
>

export function initResetTokenDestinationField(
    options: Atom<LoadState<readonly ResetTokenDestinationType[]>>,
): [ResetTokenDestinationField, BoardInitializer<ResetTokenDestination>] {
    const type = initSelectFieldBoard(options, { convert: (data) => data })
    const email = initTextFieldBoard({ convert: resetTokenDestinationEmailConverter })
    email[0] = {
        ...email[0],
        validate: combineAtom(type[0].value, email[0].validate, (type, email) => {
            if (type === "email") {
                return email
            } else {
                return { valid: true, value: restoreResetTokenDestinationEmail("") }
            }
        }),
        observe: combineAtom(type[0].value, email[0].observe, (type, email) => {
            if (type === "email") {
                return email
            } else {
                return { hasChanged: false }
            }
        }),
    }

    return [
        {
            type: type[0],
            email: email[0],
            validate: combineAtom(
                type[0].validate,
                email[0].validate,
                (type, email): ValidateResetTokenDestinationValue => {
                    if (!type.valid) {
                        return { valid: false, err: { type: "type", err: type.err } }
                    }
                    if (!email.valid) {
                        return { valid: false, err: { type: "email", err: email.err } }
                    }
                    switch (type.value) {
                        case "none":
                            return { valid: true, value: { type: "none" } }

                        case "email":
                            return { valid: true, value: { type: "email", email: email.value } }
                    }
                },
            ),
            observe: composeAtom([type[0].observe, email[0].observe], (state) => {
                return { hasChanged: state.some((state) => state.hasChanged) }
            }),
        },
        {
            init(value) {
                type[1].init(value.type)
                email[1].init(
                    value.type === "email" ? value.email : restoreResetTokenDestinationEmail(""),
                )
            },
            reset() {
                type[1].reset()
                email[1].reset()
            },
            pin() {
                type[1].pin()
                email[1].pin()
            },
        },
    ]
}
