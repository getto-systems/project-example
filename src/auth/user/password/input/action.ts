import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"
import {
    initTextFieldActionWithResource,
    TextFieldAction,
    TextFieldActionSubscriber,
} from "../../../../z_lib/ui/input/field/text"

import { passwordBoardConverter } from "./convert"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { Password } from "./data"

export interface PasswordFieldAction extends TextFieldAction<Password> {
    readonly character: PasswordCharacterAction
}

export type PasswordCharacterAction = StatefulApplicationAction<PasswordCharacterState>
export type PasswordCharacterState = Readonly<{ multiByte: boolean }>

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

class CharacterAction
    extends AbstractStatefulApplicationAction<PasswordCharacterState>
    implements PasswordCharacterAction
{
    readonly initialState: PasswordCharacterState = { multiByte: false }

    infra: PasswordCharacterInfra

    constructor(infra: PasswordCharacterInfra) {
        super()

        this.infra = infra

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
