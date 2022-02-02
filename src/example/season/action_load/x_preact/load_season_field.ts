import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { field } from "../../../../../ui/vendor/getto-css/preact/design/form"
import { notice_alert } from "../../../../../ui/vendor/getto-css/preact/design/highlight"
import { v_small } from "../../../../../ui/vendor/getto-css/preact/design/alignment"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"

import { LoadSeasonResource, LoadSeasonResourceState } from "../resource"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"
import { seasonLabel } from "../../kernel/helper"

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