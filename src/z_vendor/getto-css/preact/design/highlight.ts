import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../common"

export function linky(content: PreactContent): PreactNode {
    return html`<span class="linky">${content}</span>`
}

type Color = "gray" | "alert" | "success" | "warning" | "pending" | "info"

export function notice_gray(content: PreactContent): PreactNode {
    return notice("gray", content)
}
export function notice_alert(content: PreactContent): PreactNode {
    return notice("alert", content)
}
export function notice_success(content: PreactContent): PreactNode {
    return notice("success", content)
}
export function notice_warning(content: PreactContent): PreactNode {
    return notice("warning", content)
}
export function notice_pending(content: PreactContent): PreactNode {
    return notice("pending", content)
}
export function notice_info(content: PreactContent): PreactNode {
    return notice("info", content)
}
function notice(color: Color, content: PreactContent): PreactNode {
    return html`<p class="notice notice_${color}">${content}</p>`
}

export function label_gray(content: PreactContent): PreactNode {
    return label("gray", content)
}
export function label_alert(content: PreactContent): PreactNode {
    return label("alert", content)
}
export function label_success(content: PreactContent): PreactNode {
    return label("success", content)
}
export function label_warning(content: PreactContent): PreactNode {
    return label("warning", content)
}
export function label_pending(content: PreactContent): PreactNode {
    return label("pending", content)
}
export function label_info(content: PreactContent): PreactNode {
    return label("info", content)
}
function label(color: Color, content: PreactContent): PreactNode {
    return html`<span class="label label_${color}">${content}</span>`
}

export function badge_gray(content: PreactContent): PreactNode {
    return badge("gray", content)
}
export function badge_alert(content: PreactContent): PreactNode {
    return badge("alert", content)
}
export function badge_success(content: PreactContent): PreactNode {
    return badge("success", content)
}
export function badge_warning(content: PreactContent): PreactNode {
    return badge("warning", content)
}
export function badge_pending(content: PreactContent): PreactNode {
    return badge("pending", content)
}
export function badge_info(content: PreactContent): PreactNode {
    return badge("info", content)
}
function badge(color: Color, content: PreactContent): PreactNode {
    return html`<span class="badge badge_${color}">${content}</span>`
}
