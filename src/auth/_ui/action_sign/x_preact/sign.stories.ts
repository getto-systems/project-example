import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { SignComponent } from "./sign"

import { mockSignAction } from "../mock"

import { SignActionState } from "../action"

export default {
    title: "main/Auth/Sign/Error",
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
