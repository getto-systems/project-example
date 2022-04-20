import { VNode } from "preact"
import { useLayoutEffect, useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent, VNodeKey } from "../../../../getto-css/preact/common"
import { checkbox, checkbox_block } from "../../../../getto-css/preact/design/form"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector, MultipleBoardValueStore } from "../infra"

export type CheckboxBoardContent = Readonly<{
    key: VNodeKey
    value: string
    label: VNodeContent
}>

type Props = Readonly<{
    input: InputBoardAction<MultipleBoardValueStore>
    options: readonly CheckboxBoardContent[]
}> &
    Partial<{
        block: boolean
    }>
export function CheckboxBoard(props: Props): VNode {
    const [current, store] = useCheckboxStore(props.input.connector)

    return html`${content()}`

    function content(): readonly VNode[] {
        return props.options.map(({ key, value, label }) => {
            const isChecked = current.has(value)

            const input = html`<input
                    type="checkbox"
                    checked=${isChecked}
                    onInput=${onInput}
                />${label}`

            const content = { isChecked, input, key }

            if (props.block) {
                return checkbox_block(content)
            } else {
                return checkbox(content)
            }

            function onInput(e: Event) {
                const target = e.target
                if (target instanceof HTMLInputElement) {
                    store.setChecked(value, target.checked)
                }
                props.input.publisher.post()
            }
        })
    }
}

interface CheckboxStore {
    setChecked(value: string, isChecked: boolean): void
}

function useCheckboxStore(
    connector: BoardValueStoreConnector<MultipleBoardValueStore>,
): [ReadonlySet<string>, CheckboxStore] {
    const [current, setValue] = useState<ReadonlySet<string>>(new Set())
    const store = useMemo(() => new ValuesStore(), [])

    useLayoutEffect(() => {
        store.connect(setValue)
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store])

    return [current, store]
}

class ValuesStore implements CheckboxStore {
    value: Set<string>

    constructor() {
        this.value = new Set()
    }

    setValue: { (value: ReadonlySet<string>): void } = () => null

    connect(setValue: { (value: ReadonlySet<string>): void }): void {
        setValue(this.value)
        this.setValue = setValue
    }

    setChecked(value: string, isChecked: boolean): void {
        if (isChecked) {
            this.value.add(value)
        } else {
            this.value.delete(value)
        }
        this.setValue(this.value)
    }

    get(): readonly string[] {
        return Array.from(this.value.values())
    }
    set(value: readonly string[]): void {
        this.value = new Set(value)
        this.setValue(this.value)
    }
}
