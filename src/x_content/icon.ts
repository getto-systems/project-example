import { html } from "htm/preact"

import { VNodeContent } from "../z_lib/ui/x_preact/common"

import { iconHtml } from "../core/x_preact/design/icon"
import { lnir } from "../z_lib/ui/icon/init/line_icon"

import { Icon } from "../z_lib/ui/icon/data"

export const icon_home: Icon = lnir(["flags"])
export const icon_spinner: Icon = lnir(["spinner", "is-spinning"])
export const icon_search: Icon = lnir(["search"])
export const icon_reload: Icon = lnir(["spinner-arrow"])
export const icon_edit: Icon = lnir(["pencil"])
export const icon_edit_focused: Icon = lnir(["pencil-alt"])
export const icon_save: Icon = lnir(["bolt"])
export const icon_sidebar_fold: Icon = lnir(["shift-right"])
export const icon_sidebar_expand: Icon = lnir(["shift-left"])

export function buttonLabel(
    label: VNodeContent,
    icon: Icon,
): Readonly<{ static: VNodeContent; connect: VNodeContent }> {
    return {
        static: html`${label} ${iconHtml(icon)}`,
        connect: html`${label} ${iconHtml(icon_spinner)}`,
    }
}
