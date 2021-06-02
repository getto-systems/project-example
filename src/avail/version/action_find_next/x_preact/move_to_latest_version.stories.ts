import { h } from "preact"

import { enumKeys, storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { mockFindNextVersionCoreAction } from "../core/mock"

import { MoveToLatestVersionComponent } from "./move_to_latest_version"

import { FindNextVersionCoreState } from "../core/action"

enum FindNextEnum {
    "takeLongtime",
    "failed",
}

export default {
    title: "main/public/Avail/Move To Latest Version",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        findNext: {
            control: { type: "select", options: enumKeys(FindNextEnum) },
        },
    },
}

type MockProps = Readonly<{
    findNext: keyof typeof FindNextEnum
    err: string
}>
const template = storyTemplate<MockProps>((props) => {
    return h(MoveToLatestVersionComponent, {
        findNext: mockFindNextVersionCoreAction(),
        state: state(),
    })

    function state(): FindNextVersionCoreState {
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
