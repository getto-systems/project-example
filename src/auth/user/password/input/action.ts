import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"
import {
    initTextFieldActionWithResource,
    TextFieldAction,
} from "../../../../z_lib/ui/input/field/text"
import { TextFieldActionSubscriber } from "../../../../z_lib/ui/input/field/init/pubsub"

import { passwordBoardConverter } from "./convert"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { Password } from "./data"

export interface PasswordFieldAction extends TextFieldAction<Password> {
    readonly character: PasswordCharacterAction
}

export interface PasswordCharacterAction {
    readonly state: ApplicationState<PasswordCharacterState>
}
export type PasswordCharacterState = Readonly<{ multiByte: boolean }>

const initialState: PasswordCharacterState = { multiByte: false }

export function initPasswordFieldAction(): PasswordFieldAction {
    return initTextFieldActionWithResource({
        convert: passwordBoardConverter,
        resource: (infra) => ({
            character: initPasswordCharacterAction(infra),
        }),
    })
}

type PasswordCharacterInfra = Readonly<{
    store: BoardValueStore
    subscriber: TextFieldActionSubscriber
}>

function initPasswordCharacterAction(infra: PasswordCharacterInfra): PasswordCharacterAction {
    const { state, post } = initApplicationState({ initialState })

    infra.subscriber.subscribe({
        onInput: check,
        onClear: check,
        onReset: check,
    })

    return {
        state,
    }

    function check(): PasswordCharacterState {
        const value = infra.store.get()
        return post({
            multiByte: new TextEncoder().encode(value).byteLength > value.length,
        })
    }
}
