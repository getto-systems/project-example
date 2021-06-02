import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { field } from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../../../ui/vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../../../ui/vendor/getto-css/preact/design/alignment"

import { VNodeContent } from "../../../x_preact/design/common"

import { LoadSeasonResource, LoadSeasonResourceState } from "../resource"

import { RepositoryError } from "../../../../../../ui/vendor/getto-application/infra/repository/data"
import { Season } from "../../load_season/data"

export function LoadSeasonFieldEntry(resource: LoadSeasonResource): VNode {
    return h(LoadSeasonFieldComponent, {
        ...resource,
        state: useApplicationAction(resource.season),
    })
}

type Props = LoadSeasonResource & LoadSeasonResourceState
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

    function detail(): VNode[] {
        if (err.err.length === 0) {
            return []
        }
        return [v_small(), html`<small><p>詳細: ${err.err}</p></small>`]
    }
}

const EMPTY_CONTENT = html``