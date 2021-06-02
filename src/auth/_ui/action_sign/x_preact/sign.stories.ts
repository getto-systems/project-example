import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { SignComponent } from "./sign"

import { mockSignAction } from "../core/mock"

import { SignActionState } from "../core/action"

export default {
    title: "main/public/Auth/Sign/Error",
    parameters: {
        layout: "fullscreen",
    },
}

type Props = Readonly<{
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(SignComponent, {
        sign: mockSignAction(),
        state: state(),
    })

    function state(): SignActionState {
        return { type: "error", err: props.err }
    }
})

export const Error = template({ err: "error" })
