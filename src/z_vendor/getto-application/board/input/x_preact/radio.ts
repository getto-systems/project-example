import { VNode } from "preact"
import { useLayoutEffect, useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent, VNodeKey } from "../../../../getto-css/preact/common"

import { InputBoardAction } from "../action"

import { BoardValueStore, BoardValueStoreConnector } from "../../input/infra"

import { BoardValue } from "../../kernel/data"
import { radio, radio_block } from "../../../../getto-css/preact/design/form"

export type RadioBoardContent = Readonly<{
    key: VNodeKey
    value: BoardValue
    label: VNodeContent
}>

type Props = Readonly<{
    input: InputBoardAction
    name: string
    defaultChecked: BoardValue
    options: readonly RadioBoardContent[]
}> &
    Partial<{
        block: boolean
        autoFocus: boolean
        onKeyDown: { (e: KeyboardEvent): void }
    }>
export function RadioBoardComponent(props: Props): VNode {
    const [checkedValue, store] = useRadioStore(props.input.connector, props.defaultChecked)

    return html`${content()}`

    function content(): VNode[] {
        return props.options.map(({ key, value, label }, i) => {
            const isChecked = value === checkedValue

            const input = html`<input
                    type="radio"
                    name="${props.name}"
                    checked=${isChecked}
                    onInput=${onInput}
                    onKeyDown=${props.onKeyDown}
                    autofocus=${i === 0 && props.autoFocus}
                />${label}`

            const content = { isChecked, input, key }

            if (props.block) {
                return radio_block(content)
            } else {
                return radio(content)
            }

            function onInput(e: Event) {
                const target = e.target
                if (target instanceof HTMLInputElement) {
                    if (target.checked) {
                        store.set(value)
                    }
                }
                props.input.publisher.post()
            }
        })
    }
}

interface RadioStore {
    set(value: string): void
}

function useRadioStore(
    connector: BoardValueStoreConnector,
    defaultChecked: BoardValue,
): [string, RadioStore] {
    const store = useMemo(() => new ValueStore(defaultChecked), [defaultChecked])

    useLayoutEffect(() => {
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store])

    const [value, setValue] = useState<BoardValue>(defaultChecked)
    useLayoutEffect(() => {
        store.connect(setValue)
    }, [store, setValue])

    return [value, store]
}

type PendingStore = Readonly<{ hasValue: false }> | Readonly<{ hasValue: true; value: BoardValue }>

class ValueStore implements RadioStore, BoardValueStore {
    value: BoardValue

    constructor(defaultChecked: BoardValue) {
        this.value = defaultChecked
    }

    setValue: { (value: BoardValue): void } = (value) => {
        this.pendingStore = { hasValue: true, value }
    }

    pendingStore: PendingStore = { hasValue: false }

    connect(setValue: { (value: BoardValue): void }): void {
        if (this.pendingStore.hasValue) {
            const pendingValue = this.pendingStore.value
            this.pendingStore = { hasValue: false }
            setValue(pendingValue)
        }

        this.setValue = setValue
    }

    get(): BoardValue {
        return this.value
    }
    set(value: BoardValue): void {
        this.value = value
        this.setValue(value)
    }
}
