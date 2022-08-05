import {
    ApplicationState,
    initApplicationState,
    StatefulApplicationAction,
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

export type PasswordCharacterAction = StatefulApplicationAction<PasswordCharacterState>
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
    return new CharacterAction(infra)
}

class CharacterAction implements PasswordCharacterAction {
    readonly infra: PasswordCharacterInfra
    readonly state: ApplicationState<PasswordCharacterState>
    readonly post: (state: PasswordCharacterState) => PasswordCharacterState

    constructor(infra: PasswordCharacterInfra) {
        const { state, post } = initApplicationState({ initialState })
        this.infra = infra
        this.state = state
        this.post = post

        infra.subscriber.subscribe({
            onInput: () => this.check(),
            onClear: () => this.check(),
            onReset: () => this.check(),
        })
    }

    check(): PasswordCharacterState {
        const value = this.infra.store.get()
        return this.post({
            multiByte: new TextEncoder().encode(value).byteLength > value.length,
        })
    }
}
