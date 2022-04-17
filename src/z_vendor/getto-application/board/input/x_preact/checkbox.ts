import { VNode } from "preact"
import { useLayoutEffect, useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent, VNodeKey } from "../../../../getto-css/preact/common"
import { checkbox, checkbox_block } from "../../../../getto-css/preact/design/form"

import { InputBoardAction } from "../action"

import { BoardValueStoreConnector, MultipleBoardValueStore } from "../infra"

import { BoardValue } from "../../kernel/data"

export type CheckboxBoardContent = Readonly<{
    key: VNodeKey
    value: BoardValue
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
): [ReadonlySet<BoardValue>, CheckboxStore] {
    const [current, setValue] = useState<ReadonlySet<BoardValue>>(new Set())
    const store = useMemo(() => new ValuesStore(), [])

    useLayoutEffect(() => {
        store.connect(setValue)
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store])

    return [current, store]
}

class ValuesStore implements CheckboxStore {
    value: Set<BoardValue>

    constructor() {
        this.value = new Set()
    }

    setValue: { (value: ReadonlySet<BoardValue>): void } = () => null

    connect(setValue: { (value: ReadonlySet<BoardValue>): void }): void {
        setValue(this.value)
        this.setValue = setValue
    }

    setChecked(value: BoardValue, isChecked: boolean): void {
        if (isChecked) {
            this.value.add(value)
        } else {
            this.value.delete(value)
        }
        this.setValue(this.value)
    }

    get(): readonly BoardValue[] {
        return Array.from(this.value.values())
    }
    set(value: readonly BoardValue[]): void {
        this.value = new Set(value)
        this.setValue(this.value)
    }
}
