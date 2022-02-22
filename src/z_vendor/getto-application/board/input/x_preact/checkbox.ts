import { VNode } from "preact"
import { useLayoutEffect, useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent, VNodeKey } from "../../../../getto-css/preact/common"
import { checkbox, checkbox_block } from "../../../../getto-css/preact/design/form"

import { MultipleInputBoardAction } from "../action"

import { MultipleBoardValueStoreConnector } from "../infra"

import { BoardValue } from "../../kernel/data"

export type CheckboxBoardContent = Readonly<{
    key: VNodeKey
    value: BoardValue
    label: VNodeContent
}>

type Props = Readonly<{
    input: MultipleInputBoardAction
    defaultChecked: readonly BoardValue[]
    options: readonly CheckboxBoardContent[]
}> &
    Partial<{
        block: boolean
    }>
export function CheckboxBoardComponent(props: Props): VNode {
    const store = useCheckboxStore(props.input.connector, props.defaultChecked)

    return html`${content()}`

    function content(): readonly VNode[] {
        return props.options.map(({ key, value, label }) => {
            const isChecked = store.has(value)

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
                    store.setMember(value, target.checked)
                }
                props.input.publisher.post()
            }
        })
    }
}

interface CheckboxStore {
    has(value: string): boolean
    setMember(value: string, isSelected: boolean): void
}

function useCheckboxStore(
    connector: MultipleBoardValueStoreConnector,
    defaultChecked: readonly BoardValue[],
): CheckboxStore {
    const store = useMemo(() => new ValuesStore(defaultChecked), [defaultChecked])
    useLayoutEffect(() => {
        connector.connect(store)
        return () => connector.terminate()
    }, [connector, store])

    const [values, setValues] = useState<readonly BoardValue[]>(defaultChecked)
    useLayoutEffect(() => {
        store.connect(values, setValues)
    }, [store, values, setValues])

    return store
}

type PendingStore =
    | Readonly<{ hasValue: false }>
    | Readonly<{ hasValue: true; values: readonly BoardValue[] }>

class ValuesStore implements CheckboxStore {
    values: Set<BoardValue>

    constructor(defaultChecked: readonly BoardValue[]) {
        this.values = new Set()
        defaultChecked.forEach((value) => {
            this.values.add(value)
        })
    }

    setValues: { (values: readonly BoardValue[]): void } = (values) => {
        this.pendingStore = { hasValue: true, values }
    }

    pendingStore: PendingStore = { hasValue: false }

    connect(
        values: readonly BoardValue[],
        setValues: { (values: readonly BoardValue[]): void },
    ): void {
        if (this.pendingStore.hasValue) {
            const pendingValues = this.pendingStore.values
            this.pendingStore = { hasValue: false }
            setValues(pendingValues)
        }

        this.setValues = setValues
        this.values = new Set(values)
    }

    has(value: BoardValue): boolean {
        return this.values.has(value)
    }
    setMember(value: BoardValue, isSelected: boolean): void {
        if (isSelected) {
            this.values.add(value)
        } else {
            this.values.delete(value)
        }
        this.setValues(this.get())
    }

    get(): readonly BoardValue[] {
        return Array.from(this.values.values())
    }
    set(values: readonly BoardValue[]): void {
        this.setValues(values)
        this.values = new Set(values)
    }
}
