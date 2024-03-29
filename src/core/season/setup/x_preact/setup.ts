import { PreactNode } from "../../../../common/x_preact/node"
import { h } from "preact"
import { html } from "htm/preact"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import { box } from "../../../../z_vendor/getto-css/preact/design/box"
import { fieldHelp_error } from "../../../../z_vendor/getto-css/preact/design/form"

import { SeasonField } from "../../input/field/x_preact/input"
import { EditButton } from "../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../common/x_preact/button/edit_success_button"
import { ChangeButton } from "../../../../common/x_preact/button/change_button"

import { repositoryErrorReason } from "../../../../common/util/repository/x_error/reason"

import { initAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadSeasonAction } from "../../load/action"
import { SetupSeasonAction } from "../action"

import { RepositoryError } from "../../../../common/util/repository/data"
import { ConnectState } from "../../../../common/util/connect/data"

type Props = Readonly<{
    season: LoadSeasonAction
    setup: SetupSeasonAction
}>
export function SetupSeason(props: Props): PreactNode {
    const loadSeasonState = useAtom(props.season.state)
    switch (loadSeasonState.type) {
        case "initial":
        case "failed":
            return html``
    }

    const edit = { data: loadSeasonState, editable: props.setup.editable }

    return box({
        form: true,
        title: "シーズン設定",
        body: [
            h(SeasonField, {
                title: "シーズン",
                field: props.setup.season,
                edit,
            }),
        ],
        footer: h(Footer, {}),
    })

    function Footer(_props: unknown): PreactNode {
        const editableState = useAtom(props.setup.editable.state)

        if (!editableState.isEditable) {
            return h(Edit, {})
        }
        return html`${[h(Submit, {}), h(Message, {})]}`

        function Edit(_props: unknown): PreactNode {
            const setupSeasonState = useAtom(props.setup.state)

            if (setupSeasonState.type === "success") {
                return h(EditSuccessButton, { onClick })
            } else {
                return h(EditButton, { onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.setup.editable.open()
            }
        }

        function Submit(_props: unknown): PreactNode {
            return h(ChangeButton, {
                connect: initAtom<ConnectState>({ initialState: { isConnecting: false } }).state,
                validate: props.setup.validate,
                observe: props.setup.observe,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.setup.setup()
            }
        }

        function Message(_props: unknown): PreactNode {
            const setupSeasonState = useAtom(props.setup.state)

            switch (setupSeasonState.type) {
                case "initial":
                case "success":
                    return html``

                case "failed":
                    return fieldHelp_error(repositoryError(setupSeasonState.err))
            }
        }
    }
}

function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりシーズン設定に失敗しました`,
        ...reason.detail,
    ])
}
