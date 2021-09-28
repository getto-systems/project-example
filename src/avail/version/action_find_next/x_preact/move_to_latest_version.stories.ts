import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { mockFindNextVersionAction } from "../mock"

import { MoveToLatestVersionComponent } from "./move_to_latest_version"

import { FindNextVersionState } from "../action"

const options = ["takeLongtime", "failed"] as const

export default {
    title: "main/Avail/Move To Latest Version",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        findNext: {
            control: { type: "select", options },
        },
    },
}

type MockProps = Readonly<{
    findNext: typeof options[number]
    err: string
}>
const template = storyTemplate<MockProps>((props) => {
    return h(MoveToLatestVersionComponent, {
        findNext: mockFindNextVersionAction(),
        state: state(),
    })

    function state(): FindNextVersionState {
        switch (props.findNext) {
            case "takeLongtime":
                return { type: "take-longtime-to-find" }

            case "failed":
                return {
                    type: "failed-to-find",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const MoveToLatestVersion = template({ findNext: "takeLongtime", err: "" })
