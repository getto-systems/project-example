import { h, VNode } from "preact"
import { useMemo, useState } from "preact/hooks"
import { html } from "htm/preact"

import { useInputRef } from "./hooks"

import { InputBoardAction } from "../action"
import { initBoardValueStoreConnector } from "../init/connector"

import { BoardValueStore, BoardValueStoreConnector, MultipleBoardValueStore } from "../infra"

export const vectorInputTypes = [
    "text",
    "search",
    "number",
    "tel",
    "email",
    "date",
    "time",
] as const
export type VectorInputType = typeof vectorInputTypes[number]

type Props = Readonly<{
    type: VectorInputType
    input: InputBoardAction<MultipleBoardValueStore>
    button: VectorButton
}>
export type VectorButton = Readonly<{
    add: (props: Readonly<{ onClick: () => void }>) => VNode
    remove: (props: Readonly<{ onClick: () => void }>) => VNode
    undoRemove: (props: Readonly<{ onClick: () => void }>) => VNode
}>
export function VectorBoard({ type, input, button }: Props): VNode {
    const [connectors, vector] = useVectorConnector(input.connector)
    return html`<ul>
        ${connectors.map((item, index) => {
            return html`<li key=${index}>
                ${[
                    h(VectorInputBoard, {
                        type,
                        connector: item.connector,
                        disabled: item.isDeleting,
                        onInput,
                    }),
                    " ",
                    item.isDeleting ? undoRemoveButton() : removeButton(),
                ]}
            </li>`

            function removeButton(): VNode {
                return h(button.remove, { onClick })

                function onClick() {
                    vector.remove(index)
                }
            }
            function undoRemoveButton(): VNode {
                return h(button.undoRemove, { onClick })

                function onClick() {
                    vector.undoRemove(index)
                }
            }
        })}
        <li>${addButton()}</li>
    </ul>`

    function onInput() {
        input.publisher.post()
    }

    function addButton(): VNode {
        return h(button.add, { onClick })

        function onClick() {
            vector.push()
        }
    }
}

type InputProps = Readonly<{
    type: VectorInputType
    connector: BoardValueStoreConnector<BoardValueStore>
    disabled: boolean
    onInput: () => void
}>
function VectorInputBoard({ type, connector, disabled, onInput }: InputProps): VNode {
    return html`<input
        ref=${useInputRef(connector)}
        type=${type}
        disabled=${disabled}
        onInput=${onInput}
    />`
}

interface Vector {
    push(): void
    remove(index: number): void
    undoRemove(index: number): void
}

function useVectorConnector(
    connector: BoardValueStoreConnector<MultipleBoardValueStore>,
): [readonly VectorItem[], Vector] {
    const [connectors, setConnectors] = useState<readonly VectorItem[]>([])
    const store = useMemo(() => new VectorStore(connector, setConnectors), [connector])
    return [connectors, store]
}

type VectorItem = Readonly<{
    connector: BoardValueStoreConnector<BoardValueStore>
    store: BoardValueStore
    isDeleting: boolean
}>

class VectorStore implements Vector {
    store: VectorItem[] = []
    length = 0

    readonly setConnectors: (connectors: readonly VectorItem[]) => void

    constructor(
        connector: BoardValueStoreConnector<MultipleBoardValueStore>,
        setConnectors: (connectors: readonly VectorItem[]) => void,
    ) {
        this.setConnectors = setConnectors

        connector.connect({
            get: () => {
                return this.current()
                    .filter((item) => !item.isDeleting)
                    .map((item) => item.store.get())
            },
            set: (value) => {
                this.extend(value.length)
                value.forEach((value, index) => {
                    this.store[index].store.set(value)
                    this.store[index] = { ...this.store[index], isDeleting: false }
                })
            },
        })
    }

    current(): readonly VectorItem[] {
        return this.store.slice(0, this.length)
    }
    postCurrent(): void {
        this.setConnectors(this.current())
    }

    extend(length: number): void {
        while (this.store.length < length) {
            // infra の実装扱いなので、例外的に init を使用していい
            this.store.push({
                ...initBoardValueStoreConnector(),
                isDeleting: false,
            })
        }

        this.length = length
    }

    push(): void {
        this.extend(this.length + 1)
        this.postCurrent()
    }
    remove(index: number): void {
        this.setDeleting(index, true)
    }
    undoRemove(index: number): void {
        this.setDeleting(index, false)
    }
    setDeleting(index: number, isDeleting: boolean): void {
        if (index < 0 || index >= this.store.length) {
            return
        }
        this.store[index] = { ...this.store[index], isDeleting }
        this.postCurrent()
    }
}
