import {
    DocsAction,
    DocsActionContent,
    DocsActionTargetType,
    DocsAction_action,
    DocsAction_request,
    DocsContent,
    DocsDescription,
    DocsNegativeNote,
    DocsSection,
} from "./data"

export function docsSection(title: string, body: DocsContent[]): DocsSection {
    return { type: "normal", title, body }
}
export function docsSection_pending(title: string, body: DocsContent[]): DocsSection {
    return { type: "pending", title, body }
}
export function docsSection_double(title: string, body: DocsContent[]): DocsSection {
    return { type: "double", title, body }
}

export function docsPurpose(content: string[]): DocsContent {
    return { type: "purpose", content }
}
export function docsModule(content: string[]): DocsContent {
    return { type: "module", content }
}
export function docsItem(title: string, content: string[]): DocsContent {
    return { type: "item", title, content }
}
export function docsDescription(content: DocsDescription[]): DocsContent {
    return { type: "description", content }
}
export function docsExplanation(target: DocsActionTargetType[]): DocsContent {
    return { type: "explanation", target }
}
export function docsNegativeNote(content: DocsNegativeNote[]): DocsContent {
    return { type: "negativeNote", content }
}
export function docsAction(content: { (factory: DocsActionFactory): DocsAction[] }): DocsContent {
    return { type: "action", content: content({ request, action, message, validate }) }

    function request(content: DocsAction_request): DocsAction {
        return { type: "request", content }
    }
    function action(content: DocsAction_action): DocsAction {
        return { type: "action", content }
    }
    function message(messages: string[]): DocsActionContent[] {
        return messages.map((message) => ({ type: "normal", message }))
    }
    function validate(messages: string[]): DocsActionContent[] {
        return messages.map((message) => ({ type: "validate", message }))
    }
}
export function docsNote(content: string[]): DocsContent {
    return { type: "note", content }
}

export interface DocsActionFactory {
    request(content: DocsAction_request): DocsAction
    action(content: DocsAction_action): DocsAction
    message(messages: string[]): DocsActionContent[]
    validate(messages: string[]): DocsActionContent[]
}
