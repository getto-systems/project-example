import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { box } from "../../../../z_vendor/getto-css/preact/design/box"
import { button_edit, field, fieldError } from "../../../../z_vendor/getto-css/preact/design/form"

import { InputSeasonComponent } from "../../input/x_preact/input"

import { repositoryErrorReason } from "../../../../z_lib/ui/repository/x_error/reason"
import { seasonLabel } from "../../kernel/helper"

import { Season } from "../../kernel/data"
import { RepositoryError } from "../../../../z_lib/ui/repository/data"
import { LoadSeasonAction, LoadSeasonState } from "../../load/action"
import { SetupSeasonAction, SetupSeasonState } from "../action"

type EntryProps = Readonly<{
    season: LoadSeasonAction
    setupSeason: SetupSeasonAction
}>
export function SetupSeasonEntry({ season, setupSeason }: EntryProps): VNode {
    return h(SetupSeasonComponent, {
        season,
        setupSeason,
        state: useApplicationAction(setupSeason),
        load: useApplicationAction(season),
    })
}

type Props = EntryProps & Readonly<{ load: LoadSeasonState; state: SetupSeasonState }>
export function SetupSeasonComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, load }: Props): VNode {
        switch (load.type) {
            case "initial":
            case "failed":
                return EMPTY_CONTENT

            case "success":
                switch (state.type) {
                    case "initial":
                    case "success":
                        return seasonBox({ season: load.season })

                    case "edit-season":
                        return seasonForm({ seasons: load.availableSeasons })

                    case "invalid":
                        return errorMessage({ err: ["シーズンの設定に失敗しました"] })

                    case "failed":
                        return errorMessage({ err: repositoryError(state.err) })
                }
        }
    }

    function title() {
        return "シーズン設定"
    }

    type BoxContent = Readonly<{ season: Season }>

    function seasonBox({ season }: BoxContent): VNode {
        return box({
            title: title(),
            body: [
                field({
                    title: "シーズン",
                    body: seasonLabel(season),
                }),
            ],
            footer: button_edit({ state: "normal", label: "変更", onClick }),
            form: true,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.setupSeason.open()
        }
    }

    type FormContent = Readonly<{ seasons: readonly Season[] }>

    function seasonForm({ seasons }: FormContent): VNode {
        return box({
            body: [
                h(InputSeasonComponent, {
                    title: "シーズン",
                    field: props.setupSeason.season,
                    seasons,
                }),
            ],
            footer: button_edit({ state: "confirm", label: "変更", onClick }),
            form: true,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.setupSeason.setup()
        }
    }

    type ErrorContent = Readonly<{ err: readonly string[] }>
    function errorMessage({ err }: ErrorContent): VNode {
        return box({ body: fieldError(err) })
    }
}

const EMPTY_CONTENT = html``

function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりシーズン設定に失敗しました`,
        ...reason.detail,
    ])
}
