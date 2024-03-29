import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../common"

export type ReportContent = Readonly<{
    id: string
    header: PreactContent
    body: PreactContent
    footer: PreactContent
}>

type ReportStyle = Readonly<{ size: ReportPageSize; layout: ReportLayout }>
type ReportPageSize = "a4"
type ReportLayout = "portrait"
function reportClass({ size, layout }: ReportStyle) {
    return `report_${size}_${layout}`
}

// id 付与 : ルート要素
export function report_a4_portrait(content: ReportContent): PreactNode {
    return reportContent({ size: "a4", layout: "portrait" }, content)
}
function reportContent(
    style: ReportStyle,
    { id, header, body, footer }: ReportContent,
): PreactNode {
    return html`<article class="report ${reportClass(style)}" id=${id} key=${id}>
        <main>
            ${contentLimitMarker()}
            <header class="report__header">${header}</header>
            <section>${body}</section>
        </main>
        <footer class="report__footer">${footer}</footer>
    </article>`

    function contentLimitMarker() {
        return html`<aside class="report__contentLimit">
            <mark class="report__contentLimit__mark"></mark>
        </aside>`
    }
}

export type ReportTitleContent = Readonly<{
    style: ReportTitleStyle
    title: PreactContent
}>

type ReportTitleTypedContent = Readonly<{
    type: ReportTitleType
    content: ReportTitleContent
}>

type ReportTitleType = "large" | "small" | "xSmall"
type ReportTitleStyle = "left" | "center"

function mapReportTitleType(type: ReportTitleType): string {
    return `report__title_${type}`
}
function mapReportTitleStyle(style: ReportTitleStyle): string {
    switch (style) {
        case "left":
            return ""

        default:
            return `report__title_${style}`
    }
}

export function reportTitle(content: ReportTitleContent): PreactNode {
    return reportTitleContent({ type: "large", content })
}
export function reportTitle_small(content: ReportTitleContent): PreactNode {
    return reportTitleContent({ type: "small", content })
}
export function reportTitle_xSmall(content: ReportTitleContent): PreactNode {
    return reportTitleContent({ type: "xSmall", content })
}
function reportTitleContent(report: ReportTitleTypedContent): PreactNode {
    return html`<h1 class="${titleClass()}">${report.content.title}</h1>`

    function titleClass(): string {
        return [
            "report__title",
            mapReportTitleType(report.type),
            mapReportTitleStyle(report.content.style),
        ].join(" ")
    }
}

export type ReportFoliosContent = Readonly<{
    left: PreactContent
    right: PreactContent
}>
export function reportFolios({ left, right }: ReportFoliosContent): PreactNode {
    return html`<section class="report__folio__container">
        <aside class="report__folio_left">${left}</aside>
        <aside class="report__folio_right">${right}</aside>
    </section>`
}

export function reportFolio(content: PreactContent): PreactNode {
    return html`<address class="report__folio">${content}</address>`
}
