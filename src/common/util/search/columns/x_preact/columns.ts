import { h } from "preact"
import { PreactContent, PreactNode } from "../../../../x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { CheckboxBoard } from "../../../board/input/x_preact/checkbox"
import { field, field_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { tableViewColumns } from "../../../../../z_vendor/getto-css/preact/design/table"
import { checkboxOptions } from "../../../../x_preact/design/checkbox"
import { repositoryErrorReason } from "../../../repository/x_error/reason"

import { SearchColumnsBoard } from "../action"

import { RepositoryError } from "../../../repository/data"

export function SearchColumns(
    props: Readonly<{
        filter: SearchColumnsBoard
    }> &
        Partial<{
            title: PreactContent
            block: boolean
        }>,
): PreactNode {
    const columnsState = useAtom(props.filter.state)
    switch (columnsState.type) {
        case "success":
            return field({
                title: title(),
                body: [h(Checkbox, {})],
            })

        case "repository-error":
            return field_error({
                title: title(),
                body: [h(Checkbox, {})],
                notice: repositoryError(columnsState.err),
            })
    }

    function title(): PreactContent {
        if (props.title) {
            return props.title
        }
        return "表示する列"
    }
    function Checkbox(_props: unknown): PreactNode {
        const options = useAtom(props.filter.options)

        return tableViewColumns(
            h(CheckboxBoard, {
                input: props.filter.input,
                options: checkboxOptions(options, (column) => ({
                    key: column.key,
                    value: `${column.key}`,
                    label: column.content,
                })),
                block: block(),
            }),
        )

        function block(): boolean {
            if (props.block) {
                return props.block
            }
            return false
        }
    }
}
function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりカラムの選択に失敗しました`,
        ...reason.detail,
    ])
}
