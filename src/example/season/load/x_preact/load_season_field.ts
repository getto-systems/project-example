import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { field } from "../../../../z_vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../z_vendor/getto-css/preact/design/alignment"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"

import { seasonLabel } from "../../kernel/helper"

import { LoadSeasonAction, LoadSeasonState } from "../action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"

type EntryProps = Readonly<{
    season: LoadSeasonAction
}>
export function LoadSeasonFieldEntry({ season }: EntryProps): VNode {
    return h(LoadSeasonFieldComponent, { season, state: useApplicationAction(season) })
}

type Props = EntryProps & Readonly<{ state: LoadSeasonState }>
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
                return seasonLabel(props.state.season)

            case "failed-to-load":
                return loadError(props.state.err)
        }
    }
}

function loadError(err: RepositoryError): VNodeContent {
    return [notice_alert("ロードエラー"), ...detail()]

    function detail(): VNode[] {
        if (err.err.length === 0) {
            return []
        }
        return [v_small(), html`<small><p>詳細: ${err.err}</p></small>`]
    }
}

const EMPTY_CONTENT = html``
