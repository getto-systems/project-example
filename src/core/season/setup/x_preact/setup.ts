import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { box } from "../../../../z_vendor/getto-css/preact/design/box"
import { button_edit, field, fieldError } from "../../../../z_vendor/getto-css/preact/design/form"

import { InputSeason } from "../../input/x_preact/input"

import { repositoryErrorReason } from "../../../../z_lib/ui/repository/x_error/reason"
import { seasonLabel } from "../../kernel/helper"

import { LoadSeasonAction } from "../../load/action"
import { SetupSeasonAction } from "../action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"

type Props = Readonly<{
    season: LoadSeasonAction
    setupSeason: SetupSeasonAction
}>
export function SetupSeason(props: Props): VNode {
    const state = useApplicationAction(props.setupSeason)
    const loadState = useApplicationAction(props.season)

    const content = {
        title: "シーズン設定",
    }

    switch (loadState.type) {
        case "initial":
        case "failed":
            return EMPTY_CONTENT

        case "success":
            switch (state.type) {
                case "initial":
                case "success":
                    return box({
                        ...content,
                        body: [
                            field({
                                title: "シーズン",
                                body: seasonLabel(loadState.season),
                            }),
                        ],
                        footer: openButton(),
                        form: true,
                    })

                case "edit-season":
                    return box({
                        ...content,
                        body: [
                            h(InputSeason, {
                                title: "シーズン",
                                field: props.setupSeason.season,
                                seasons: loadState.availableSeasons,
                            }),
                        ],
                        footer: setupButton(),
                        form: true,
                    })

                case "invalid":
                    return box({ body: fieldError(["シーズンの設定に失敗しました"]) })

                case "failed":
                    return box({ body: fieldError(repositoryError(state.err)) })
            }
    }

    function openButton(): VNode {
        return button_edit({ state: "normal", label: "変更", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.setupSeason.open()
        }
    }

    function setupButton(): VNode {
        return button_edit({ state: "confirm", label: "変更", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.setupSeason.setup()
        }
    }
}

const EMPTY_CONTENT = html``

function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりシーズン設定に失敗しました`,
        ...reason.detail,
    ])
}
