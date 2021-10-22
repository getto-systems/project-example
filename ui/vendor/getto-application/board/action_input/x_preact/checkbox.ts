import { VNode } from "preact"
import { useLayoutEffect, useState } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../getto-css/preact/common"
import { checkbox, checkbox_block } from "../../../../getto-css/preact/design/form"

import { MultipleInputBoardAction } from "../action"

import { MultipleBoardValueStoreConnector } from "../../input/infra"

import { BoardValue } from "../../kernel/data"

type Props =
    | Readonly<{
          input: MultipleInputBoardAction
          options: Readonly<{ key: string; value: BoardValue; label: VNodeContent }>[]
      }>
    | Readonly<{
          input: MultipleInputBoardAction
          options: Readonly<{ key: string; value: BoardValue; label: VNodeContent }>[]
          block: boolean
      }>
export function CheckboxBoardComponent(props: Props): VNode {
    const [values, setValues] = useMultipleBoardValueState(props.input.connector)

    return html`${content()}`

    function content(): VNode[] {
        return props.options.map(({ key, value, label }) => {
            const isChecked = values.has(value)
            const content = {
                isChecked,
                input: html`<input
                        type="checkbox"
                        checked=${isChecked}
                        onInput=${onInput}
                    />${label}`,
                key,
            }

            if ("block" in props && props.block) {
                return checkbox_block(content)
            } else {
                return checkbox(content)
            }

            function onInput(e: Event) {
                const target = e.target
                if (target instanceof HTMLInputElement) {
                    if (target.checked) {
                        values.add(value)
                    } else {
                        values.delete(value)
                    }
                    setValues(values)
                    props.input.publisher.post()
                }
            }
        })
    }
}

function useMultipleBoardValueState(connector: MultipleBoardValueStoreConnector) {
    const [values, setValues] = useState<Set<BoardValue>>(new Set())

    useLayoutEffect(() => {
        connector.connect({
            get: () => Array.from(values.values()),
            set: (values) => setValues(new Set(values)),
        })
        return () => connector.terminate()
    }, [connector, values])

    return [values, setValues] as const
}
