import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../common"

export function form(content: PreactContent): PreactNode {
    return html`<form>${content}</form>`
}

type FieldContent =
    | Readonly<{ type: NormalFieldType; content: NormalFieldContent }>
    | Readonly<{ type: SearchFieldType; content: NormalFieldContent }>
    | Readonly<{ type: NoticeFieldType; content: NoticeFieldContent }>

export type NormalFieldContent = Readonly<{
    title: PreactContent
    body: PreactContent
    help?: readonly PreactContent[]
}>
export type NoticeFieldContent = NormalFieldContent & Readonly<{ notice: readonly PreactContent[] }>
export type SearchFieldContent = NormalFieldContent &
    Readonly<{ label: { (content: PreactNode): PreactNode } }>

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
            | Readonly<{ type: "error"; notice: readonly PreactContent[] }>
        label: { (content: PreactNode): PreactNode }
    }>

export function inputField(content: InputFieldContent): PreactNode {
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

export function field(content: NormalFieldContent): PreactNode {
    return fieldContent({ type: "normal", content })
}
export function field_error(content: NoticeFieldContent): PreactNode {
    return fieldContent({ type: "error", content })
}
export function field_warning(content: NoticeFieldContent): PreactNode {
    return fieldContent({ type: "warning", content })
}
export function search(content: SearchFieldContent): PreactNode {
    return content.label(fieldContent({ type: "search", content }))
}
export function search_double(content: SearchFieldContent): PreactNode {
    return content.label(fieldContent({ type: "search_double", content }))
}

function fieldContent(field: FieldContent): PreactNode {
    const help = {
        help: helpContent(),
        notice: noticeContent(),
    }
    return html`<dl class="${mapFieldType(field.type)}">
        <dt class="field__title">${field.content.title}</dt>
        <dd class="field__body">${field.content.body} ${fieldHelp(help)}</dd>
    </dl>`

    function helpContent(): readonly PreactContent[] {
        if (field.content.help) {
            return field.content.help
        }
        return []
    }
    function noticeContent(): readonly PreactContent[] {
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
    Partial<{ help: readonly PreactContent[] }>
export type NoticeFieldSectionContent = NormalFieldSectionContent &
    Readonly<{ notice: readonly PreactContent[] }>

type NormalFieldSectionContent_base = Readonly<{ body: PreactContent }>

export function fieldSection(content: NormalFieldSectionContent): PreactNode {
    return fieldSectionContent({ type: "normal", content })
}
export function fieldSection_error(content: NoticeFieldSectionContent): PreactNode {
    return fieldSectionContent({ type: "error", content })
}
export function fieldSection_warning(content: NoticeFieldSectionContent): PreactNode {
    return fieldSectionContent({ type: "warning", content })
}

function fieldSectionContent(field: FieldSectionContent): PreactNode {
    const help = {
        help: helpContent(),
        notice: noticeContent(),
    }
    return html`<section class="${mapFieldType(field.type)}">
        ${field.content.body} ${fieldHelp(help)}
    </section>`

    function helpContent(): readonly PreactContent[] {
        if (field.content.help) {
            return field.content.help
        }
        return []
    }
    function noticeContent(): readonly PreactContent[] {
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
    help?: readonly PreactContent[]
    notice?: readonly PreactContent[]
}>
export function fieldHelp(content: FieldHelpContent): PreactNode {
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

    function notice(): PreactNode[] {
        if (!content.notice) {
            return []
        }
        return content.notice.map(toFieldNotice)
    }
    function help(): PreactNode[] {
        if (!content.help) {
            return []
        }
        return content.help.map(toFieldHelp)
    }
}
export function fieldHelp_error(notice: readonly PreactContent[]): PreactNode {
    if (notice.length === 0) {
        return html``
    }
    return html`<aside class="field__help field_error">${notice.map(toFieldNotice)}</aside>`
}
export function fieldHelp_warning(notice: readonly PreactContent[]): PreactNode {
    if (notice.length === 0) {
        return html``
    }
    return html`<aside class="field__help field_warning">${notice.map(toFieldNotice)}</aside>`
}
function toFieldNotice(message: PreactContent) {
    return html`<p class="field__notice">${message}</p>`
}
function toFieldHelp(message: PreactContent) {
    return html`<p>${message}</p>`
}

export type ButtonsContent = Partial<{ left: PreactContent; right: PreactContent }>
export function buttons(content: ButtonsContent): PreactNode {
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
    label: PreactContent
    submit?: boolean
}>
type ConnectButtonContent = Readonly<{
    state: ConnectButtonState
    label: PreactContent
    submit?: boolean
}>

export type StatelessButtonContent = Readonly<{
    onClick: Handler<Event>
    label: PreactContent
}>
export type DisabledButtonContent = Readonly<{
    label: PreactContent
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

export function button_edit(content: StatefulButtonContent): PreactNode {
    return buttonContent({ type: "edit", content })
}
export function button_search(content: StatefulButtonContent): PreactNode {
    return buttonContent({ type: "search", content })
}
export function button_send(content: StatefulButtonContent): PreactNode {
    return buttonContent({ type: "send", content })
}
export function button_delete(content: StatefulButtonContent): PreactNode {
    return buttonContent({ type: "delete", content })
}
export function button_complete(content: StatefulButtonContent): PreactNode {
    return buttonContent({ type: "complete", content })
}
export function button_warning(content: StatefulButtonContent): PreactNode {
    return buttonContent({ type: "warning", content })
}
export function button_pending(content: StatefulButtonContent): PreactNode {
    return buttonContent({ type: "pending", content })
}
export function button_cancel(content: StatelessButtonContent): PreactNode {
    return buttonContent({ type: "cancel", content: { ...content, state: "normal" } })
}
export function button_close(content: StatelessButtonContent): PreactNode {
    return buttonContent({ type: "close", content: { ...content, state: "normal" } })
}
export function button_undo(content: StatelessButtonContent): PreactNode {
    return buttonContent({ type: "undo", content: { ...content, state: "normal" } })
}
export function button_redo(content: StatelessButtonContent): PreactNode {
    return buttonContent({ type: "redo", content: { ...content, state: "normal" } })
}
export function button_disabled(content: DisabledButtonContent): PreactNode {
    return buttonContent({ type: "disabled", content: { ...content, state: "normal" } })
}

function buttonContent(button: ButtonContent): PreactNode {
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

type LabelContent = Readonly<{ style: InputStyle; content: PreactContent }>

type InputStyle = "small" | "normal" | "large" | "xLarge" | "fill"
function mapInputStyle(style: InputStyle): string {
    switch (style) {
        case "normal":
            return ""

        default:
            return `input_${style}`
    }
}

export function label(content: PreactContent): PreactNode {
    return html`<label>${content}</label>`
}

export function label_number_small(content: PreactContent): PreactNode {
    return labelContent({ style: "small", content })
}
export function label_number(content: PreactContent): PreactNode {
    return labelContent({ style: "normal", content })
}
export function label_number_fill(content: PreactContent): PreactNode {
    return labelContent({ style: "fill", content })
}

export function label_email_small(content: PreactContent): PreactNode {
    return labelContent({ style: "small", content })
}
export function label_email(content: PreactContent): PreactNode {
    return labelContent({ style: "normal", content })
}
export function label_email_fill(content: PreactContent): PreactNode {
    return labelContent({ style: "fill", content })
}

export function label_text_small(content: PreactContent): PreactNode {
    return labelContent({ style: "small", content })
}
export function label_text(content: PreactContent): PreactNode {
    return labelContent({ style: "normal", content })
}
export function label_text_large(content: PreactContent): PreactNode {
    return labelContent({ style: "large", content })
}
export function label_text_xLarge(content: PreactContent): PreactNode {
    return labelContent({ style: "xLarge", content })
}
export function label_text_fill(content: PreactContent): PreactNode {
    return labelContent({ style: "fill", content })
}

export function label_password_small(content: PreactContent): PreactNode {
    return labelContent({ style: "small", content })
}
export function label_password(content: PreactContent): PreactNode {
    return labelContent({ style: "normal", content })
}
export function label_password_large(content: PreactContent): PreactNode {
    return labelContent({ style: "large", content })
}
export function label_password_xLarge(content: PreactContent): PreactNode {
    return labelContent({ style: "xLarge", content })
}
export function label_password_fill(content: PreactContent): PreactNode {
    return labelContent({ style: "fill", content })
}

export function label_search_small(content: PreactContent): PreactNode {
    return labelContent({ style: "small", content })
}
export function label_search(content: PreactContent): PreactNode {
    return labelContent({ style: "normal", content })
}
export function label_search_large(content: PreactContent): PreactNode {
    return labelContent({ style: "large", content })
}
export function label_search_xLarge(content: PreactContent): PreactNode {
    return labelContent({ style: "xLarge", content })
}
export function label_search_fill(content: PreactContent): PreactNode {
    return labelContent({ style: "fill", content })
}

export function label_textarea_small(content: PreactContent): PreactNode {
    return labelContent({ style: "small", content })
}
export function label_textarea(content: PreactContent): PreactNode {
    return labelContent({ style: "normal", content })
}
export function label_textarea_large(content: PreactContent): PreactNode {
    return labelContent({ style: "large", content })
}
export function label_textarea_xLarge(content: PreactContent): PreactNode {
    return labelContent({ style: "xLarge", content })
}
export function label_textarea_fill(content: PreactContent): PreactNode {
    return labelContent({ style: "fill", content })
}

function labelContent({ style, content }: LabelContent): PreactNode {
    return html`<label class=${mapInputStyle(style)}>${content}</label>`
}

export type CheckableContent = Readonly<{
    isChecked: boolean
    input: PreactContent
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

export function checkbox(content: CheckableContent): PreactNode {
    return checkableContent("checkbox", "inline", content)
}
export function checkbox_block(content: CheckableContent): PreactNode {
    return checkableContent("checkbox", "block", content)
}
export function radio(content: CheckableContent): PreactNode {
    return checkableContent("radio", "inline", content)
}
export function radio_block(content: CheckableContent): PreactNode {
    return checkableContent("radio", "block", content)
}
function checkableContent(
    type: CheckableType,
    style: CheckableStyle,
    { isChecked, input, key }: CheckableContent,
): PreactNode {
    const checkClass = isChecked ? "input_checked" : ""
    return html`<label class="${mapCheckableStyle(type, style)} ${checkClass}" key=${key}
        >${input}</label
    >`
}

export function pager(content: PreactContent): PreactNode {
    return html`<label class="pager">${content}</label>`
}

interface Handler<T> {
    (event: T): void
}
