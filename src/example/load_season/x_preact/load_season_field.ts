import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { field } from "../../../../ui/vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../ui/vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../ui/vendor/getto-css/preact/design/alignment"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { LoadSeasonAction, LoadSeasonState } from "../action"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { Season } from "../data"

type EntryProps = Readonly<{
    season: LoadSeasonAction
}>
export function LoadSeasonFieldEntry({ season }: EntryProps): VNode {
    return h(LoadSeasonFieldComponent, {
        season,
        state: useApplicationAction(season),
    })
}

type Props = EntryProps &
    Readonly<{
        state: LoadSeasonState
    }>
export function LoadSeasonFieldComponent(props: Props): VNode {
    return field({
        title: "シーズン",
        body: body(),
    })

    function body(): VNodeContent {
        switch (props.state.type) {
            case "initial-season":
                return EMPTY_CONTENT

            case "succeed-to-load":
                return seasonInfo(props.state.value)

            case "failed-to-load":
                return loadError(props.state.err)
        }
    }
}

function seasonInfo(season: Season): VNodeContent {
    return season.year
}

function loadError(err: RepositoryError): VNodeContent {
    return [notice_alert("ロードエラー"), ...detail()]

    function detail(): readonly VNode[] {
        if (err.err.length === 0) {
            return []
        }
        return [v_small(), html`<small><p>詳細: ${err.err}</p></small>`]
    }
}

const EMPTY_CONTENT = html``
