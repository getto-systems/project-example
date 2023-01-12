export interface TextFieldActionSubscriber {
    subscribe(handler: TextFieldActionHandler): void
}
export type TextFieldActionHandler = Readonly<{
    onInput?: () => void
    onClear?: () => void
    onReset?: () => void
}>

export function initTextFieldActionSubscriber(): [
    TextFieldActionSubscriber,
    {
        onInput(): void
        onClear(): void
        onReset(): void
    },
] {
    let handlers: TextFieldActionHandler[] = []

    return [
        {
            subscribe: (handler) => {
                handlers = [handler, ...handlers]
            },
        },
        {
            onInput: () => {
                handlers.forEach((handler) => handler.onInput?.())
            },
            onClear: () => {
                handlers.forEach((handler) => handler.onClear?.())
            },
            onReset: () => {
                handlers.forEach((handler) => handler.onReset?.())
            },
        },
    ]
}
