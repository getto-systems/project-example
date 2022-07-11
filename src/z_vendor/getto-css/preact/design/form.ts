import { VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../common"

export function form(content: VNodeContent): VNode {
    return html`<form>${content}</form>`
}

type FieldContent =
    | Readonly<{ type: NormalFieldType; content: NormalFieldContent }>
    | Readonly<{ type: SearchFieldType; content: NormalFieldContent }>
    | Readonly<{ type: NoticeFieldType; content: NoticeFieldContent }>

export type NormalFieldContent = Readonly<{
    title: VNodeContent
    body: VNodeContent
    help?: readonly VNodeContent[]
}>
export type NoticeFieldContent = NormalFieldContent & Readonly<{ notice: readonly VNodeContent[] }>
export type SearchFieldContent = NormalFieldContent &
    Readonly<{ label: { (content: VNode): VNode } }>

type FieldType = NormalFieldType | SearchFieldType | NoticeFieldType
type NormalFieldType = "normal"
type SearchFieldType = "search" | "search_double"
type NoticeFieldType = "error" | "warning"
function mapFieldType(fieldType: FieldType): string {
    switch (fieldType) {
        case "normal":
            return ""

        case "search":
            return "search"

        case "search_double":
            return "search search_double"

        default:
            return `field_${fieldType}`
    }
}

export type InputFieldContent = NormalFieldContent &
    Readonly<{
        editableState?: Readonly<{ isEditable: boolean }>
        validateState?:
            | Readonly<{ type: "normal" }>
            | Readonly<{ type: "error"; notice: readonly VNodeContent[] }>
        label: { (content: VNode): VNode }
    }>

export function inputField(content: InputFieldContent): VNode {
    const isEditable = content.editableState === undefined ? true : content.editableState.isEditable

    if (!isEditable || content.validateState === undefined) {
        return content.label(field(content))
    }

    switch (content.validateState.type) {
        case "normal":
            return content.label(field(content))

        case "error":
            return content.label(field_error({ ...content, notice: content.validateState.notice }))
    }
}

export function field(content: NormalFieldContent): VNode {
    return fieldContent({ type: "normal", content })
}
export function field_error(content: NoticeFieldContent): VNode {
    return fieldContent({ type: "error", content })
}
export function field_warning(content: NoticeFieldContent): VNode {
    return fieldContent({ type: "warning", content })
}
export function search(content: SearchFieldContent): VNode {
    return content.label(fieldContent({ type: "search", content }))
}
export function search_double(content: SearchFieldContent): VNode {
    return content.label(fieldContent({ type: "search_double", content }))
}

function fieldContent(field: FieldContent): VNode {
    const help = {
        help: helpContent(),
        notice: noticeContent(),
    }
    return html`<dl class="${mapFieldType(field.type)}">
        <dt class="field__title">${field.content.title}</dt>
        <dd class="field__body">${field.content.body} ${fieldHelp(help)}</dd>
    </dl>`

    function helpContent(): readonly VNodeContent[] {
        if (field.content.help) {
            return field.content.help
        }
        return []
    }
    function noticeContent(): readonly VNodeContent[] {
        switch (field.type) {
            case "normal":
            case "search":
            case "search_double":
                return []

            case "error":
            case "warning":
                return field.content.notice
        }
    }
}

type FieldSectionContent =
    | Readonly<{ type: NormalFieldType; content: NormalFieldSectionContent }>
    | Readonly<{ type: NoticeFieldType; content: NoticeFieldSectionContent }>

export type NormalFieldSectionContent = NormalFieldSectionContent_base &
    Partial<{ help: readonly VNodeContent[] }>
export type NoticeFieldSectionContent = NormalFieldSectionContent &
    Readonly<{ notice: readonly VNodeContent[] }>

type NormalFieldSectionContent_base = Readonly<{ body: VNodeContent }>

export function fieldSection(content: NormalFieldSectionContent): VNode {
    return fieldSectionContent({ type: "normal", content })
}
export function fieldSection_error(content: NoticeFieldSectionContent): VNode {
    return fieldSectionContent({ type: "error", content })
}
export function fieldSection_warning(content: NoticeFieldSectionContent): VNode {
    return fieldSectionContent({ type: "warning", content })
}

function fieldSectionContent(field: FieldSectionContent): VNode {
    const help = {
        help: helpContent(),
        notice: noticeContent(),
    }
    return html`<section class="${mapFieldType(field.type)}">
        ${field.content.body} ${fieldHelp(help)}
    </section>`

    function helpContent(): readonly VNodeContent[] {
        if (field.content.help) {
            return field.content.help
        }
        return []
    }
    function noticeContent(): readonly VNodeContent[] {
        switch (field.type) {
            case "normal":
                return []

            case "error":
            case "warning":
                return field.content.notice
        }
    }
}

type FieldHelpContent = Readonly<{
    help?: readonly VNodeContent[]
    notice?: readonly VNodeContent[]
}>
export function fieldHelp(content: FieldHelpContent): VNode {
    if (helpLength() + noticeLength() === 0) {
        return html``
    }
    return html`<aside class="field__help">${notice()}${help()}</aside>`

    function helpLength(): number {
        return content.help?.length || 0
    }
    function noticeLength(): number {
        return content.notice?.length || 0
    }

    function notice(): VNode[] {
        if (!content.notice) {
            return []
        }
        return content.notice.map(toFieldNotice)
    }
    function help(): VNode[] {
        if (!content.help) {
            return []
        }
        return content.help.map(toFieldHelp)
    }
}
export function fieldHelp_error(notice: readonly VNodeContent[]): VNode {
    if (notice.length === 0) {
        return html``
    }
    return html`<aside class="field__help field_error">${notice.map(toFieldNotice)}</aside>`
}
export function fieldHelp_warning(notice: readonly VNodeContent[]): VNode {
    if (notice.length === 0) {
        return html``
    }
    return html`<aside class="field__help field_warning">${notice.map(toFieldNotice)}</aside>`
}
function toFieldNotice(message: VNodeContent) {
    return html`<p class="field__notice">${message}</p>`
}
function toFieldHelp(message: VNodeContent) {
    return html`<p>${message}</p>`
}

export type ButtonsContent = Partial<{ left: VNodeContent; right: VNodeContent }>
export function buttons(content: ButtonsContent): VNode {
    return html`<aside class="button__container">
        <section class="button_left">${left()}</section>
        <section class="button_right">${right()}</section>
    </aside>`

    function left() {
        if (content.left) {
            return content.left
        }
        return ""
    }
    function right() {
        if (content.right) {
            return content.right
        }
        return ""
    }
}

type ButtonContent =
    | Readonly<{
          type: StatefulButtonType
          content: StatefulButtonContent
      }>
    | Readonly<{
          type: StatelessButtonType
          content: StatelessButtonContent & NormalStateButtonContent
      }>
    | Readonly<{
          type: DisabledButtonType
          content: DisabledButtonContent & NormalStateButtonContent
      }>

export type StatefulButtonContent = ClickableButtonContent | ConnectButtonContent
type ClickableButtonContent = Readonly<{
    state: ClickableButtonState
    onClick: Handler<Event>
    label: VNodeContent
    submit?: boolean
}>
type ConnectButtonContent = Readonly<{
    state: ConnectButtonState
    label: VNodeContent
    submit?: boolean
}>

export type StatelessButtonContent = Readonly<{
    onClick: Handler<Event>
    label: VNodeContent
}>
export type DisabledButtonContent = Readonly<{
    label: VNodeContent
}>
type NormalStateButtonContent = Readonly<{
    state: NormalButtonState
    submit?: boolean
}>

type ButtonType = StatefulButtonType | StatelessButtonType | DisabledButtonType
type StatefulButtonType = "edit" | "search" | "send" | "delete" | "complete" | "warning" | "pending"
type StatelessButtonType = "cancel" | "close" | "undo" | "redo"
type DisabledButtonType = "disabled"

type ButtonState = ClickableButtonState | ConnectButtonState
type ClickableButtonState = NormalButtonState | "confirm"
type NormalButtonState = "normal"
type ConnectButtonState = "connect"

function mapButtonType(type: ButtonType): string {
    return `button_${type}`
}
function mapButtonState(state: ButtonState): string {
    switch (state) {
        case "normal":
            return ""

        default:
            return `button_${state}`
    }
}

export function button_edit(content: StatefulButtonContent): VNode {
    return buttonContent({ type: "edit", content })
}
export function button_search(content: StatefulButtonContent): VNode {
    return buttonContent({ type: "search", content })
}
export function button_send(content: StatefulButtonContent): VNode {
    return buttonContent({ type: "send", content })
}
export function button_delete(content: StatefulButtonContent): VNode {
    return buttonContent({ type: "delete", content })
}
export function button_complete(content: StatefulButtonContent): VNode {
    return buttonContent({ type: "complete", content })
}
export function button_warning(content: StatefulButtonContent): VNode {
    return buttonContent({ type: "warning", content })
}
export function button_pending(content: StatefulButtonContent): VNode {
    return buttonContent({ type: "pending", content })
}
export function button_cancel(content: StatelessButtonContent): VNode {
    return buttonContent({ type: "cancel", content: { ...content, state: "normal" } })
}
export function button_close(content: StatelessButtonContent): VNode {
    return buttonContent({ type: "close", content: { ...content, state: "normal" } })
}
export function button_undo(content: StatelessButtonContent): VNode {
    return buttonContent({ type: "undo", content: { ...content, state: "normal" } })
}
export function button_redo(content: StatelessButtonContent): VNode {
    return buttonContent({ type: "redo", content: { ...content, state: "normal" } })
}
export function button_disabled(content: DisabledButtonContent): VNode {
    return buttonContent({ type: "disabled", content: { ...content, state: "normal" } })
}

function buttonContent(button: ButtonContent): VNode {
    const info = detect()
    if (info.clickable) {
        return html`<button type=${info.type} class=${buttonClass()} onClick=${info.onClick}>
            ${button.content.label}
        </button>`
    } else {
        return html`<button type=${info.type} class=${buttonClass()}>
            ${button.content.label}
        </button>`
    }

    function buttonClass() {
        return `button ${mapButtonType(button.type)} ${mapButtonState(button.content.state)}`
    }

    type SubmitType = "submit" | "button"
    type Info =
        | Readonly<{ clickable: false; type: SubmitType }>
        | Readonly<{ clickable: true; type: SubmitType; onClick: Handler<Event> }>
    function detect(): Info {
        const type = submitType()
        if (button.type === "disabled") {
            return { clickable: false, type }
        }
        switch (button.content.state) {
            case "connect":
                // connect はクリックできないので button
                return { clickable: false, type: "button" }

            default:
                return { clickable: true, type, onClick: button.content.onClick }
        }

        function submitType(): SubmitType {
            if (button.content.submit !== undefined) {
                return button.content.submit ? "submit" : "button"
            }

            switch (button.type) {
                // 正常実行系は submit
                case "edit":
                case "send":
                case "search":
                case "complete":
                    return "submit"

                // 警告系、キャンセル、その他操作は button
                case "delete":
                case "warning":
                case "pending":
                case "cancel":
                case "close":
                case "undo":
                case "redo":
                case "disabled":
                    return "button"
            }
        }
    }
}

type LabelContent = Readonly<{ style: InputStyle; content: VNodeContent }>

type InputStyle = "small" | "normal" | "large" | "xLarge" | "fill"
function mapInputStyle(style: InputStyle): string {
    switch (style) {
        case "normal":
            return ""

        default:
            return `input_${style}`
    }
}

export function label(content: VNodeContent): VNode {
    return html`<label>${content}</label>`
}

export function label_number_small(content: VNodeContent): VNode {
    return labelContent({ style: "small", content })
}
export function label_number(content: VNodeContent): VNode {
    return labelContent({ style: "normal", content })
}
export function label_number_fill(content: VNodeContent): VNode {
    return labelContent({ style: "fill", content })
}

export function label_email_small(content: VNodeContent): VNode {
    return labelContent({ style: "small", content })
}
export function label_email(content: VNodeContent): VNode {
    return labelContent({ style: "normal", content })
}
export function label_email_fill(content: VNodeContent): VNode {
    return labelContent({ style: "fill", content })
}

export function label_text_small(content: VNodeContent): VNode {
    return labelContent({ style: "small", content })
}
export function label_text(content: VNodeContent): VNode {
    return labelContent({ style: "normal", content })
}
export function label_text_large(content: VNodeContent): VNode {
    return labelContent({ style: "large", content })
}
export function label_text_xLarge(content: VNodeContent): VNode {
    return labelContent({ style: "xLarge", content })
}
export function label_text_fill(content: VNodeContent): VNode {
    return labelContent({ style: "fill", content })
}

export function label_password_small(content: VNodeContent): VNode {
    return labelContent({ style: "small", content })
}
export function label_password(content: VNodeContent): VNode {
    return labelContent({ style: "normal", content })
}
export function label_password_large(content: VNodeContent): VNode {
    return labelContent({ style: "large", content })
}
export function label_password_xLarge(content: VNodeContent): VNode {
    return labelContent({ style: "xLarge", content })
}
export function label_password_fill(content: VNodeContent): VNode {
    return labelContent({ style: "fill", content })
}

export function label_search_small(content: VNodeContent): VNode {
    return labelContent({ style: "small", content })
}
export function label_search(content: VNodeContent): VNode {
    return labelContent({ style: "normal", content })
}
export function label_search_large(content: VNodeContent): VNode {
    return labelContent({ style: "large", content })
}
export function label_search_xLarge(content: VNodeContent): VNode {
    return labelContent({ style: "xLarge", content })
}
export function label_search_fill(content: VNodeContent): VNode {
    return labelContent({ style: "fill", content })
}

export function label_textarea_small(content: VNodeContent): VNode {
    return labelContent({ style: "small", content })
}
export function label_textarea(content: VNodeContent): VNode {
    return labelContent({ style: "normal", content })
}
export function label_textarea_large(content: VNodeContent): VNode {
    return labelContent({ style: "large", content })
}
export function label_textarea_xLarge(content: VNodeContent): VNode {
    return labelContent({ style: "xLarge", content })
}
export function label_textarea_fill(content: VNodeContent): VNode {
    return labelContent({ style: "fill", content })
}

function labelContent({ style, content }: LabelContent): VNode {
    return html`<label class=${mapInputStyle(style)}>${content}</label>`
}

export type CheckableContent = Readonly<{
    isChecked: boolean
    input: VNodeContent
    key: CheckableKey
}>
type CheckableKey = string | number

type CheckableType = "checkbox" | "radio"
type CheckableStyle = "inline" | "block"
function mapCheckableStyle(type: CheckableType, style: CheckableStyle): string {
    switch (style) {
        case "inline":
            return `input__${type}`

        case "block":
            return `input__${type} input__${type}_block`
    }
}

export function checkbox(content: CheckableContent): VNode {
    return checkableContent("checkbox", "inline", content)
}
export function checkbox_block(content: CheckableContent): VNode {
    return checkableContent("checkbox", "block", content)
}
export function radio(content: CheckableContent): VNode {
    return checkableContent("radio", "inline", content)
}
export function radio_block(content: CheckableContent): VNode {
    return checkableContent("radio", "block", content)
}
function checkableContent(
    type: CheckableType,
    style: CheckableStyle,
    { isChecked, input, key }: CheckableContent,
): VNode {
    const checkClass = isChecked ? "input_checked" : ""
    return html`<label class="${mapCheckableStyle(type, style)} ${checkClass}" key=${key}
        >${input}</label
    >`
}

export function pager(content: VNodeContent): VNode {
    return html`<label class="pager">${content}</label>`
}

interface Handler<T> {
    (event: T): void
}
