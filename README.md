``` ahk
!j:: {
    A_Clipboard := ""
    Send "^c"
    ClipWait
    A_Clipboard := prettify(A_Clipboard)
    Send "^v"
}

PJ_PATH := "json_prettier.dll"
JsonPrettier := DllCall("LoadLibrary", "Str", PJ_PATH, "Ptr")
prepare_proc := DllCall("GetProcAddress", "Ptr", JsonPrettier, "AStr", "prepare", "Ptr")
prettify_proc := DllCall("GetProcAddress", "Ptr", JsonPrettier, "AStr", "prettify", "Ptr")

prettify(json) {
    global prettify_proc
    json := StrReplace(json, '`r`n')
    json := StrReplace(json, '`t')
    json := StrReplace(json, '\"', '"')
    json := StrReplace(json, '\\', '\')
    required_bytes := StrPut(json, "UTF-8")
    json_utf8 := Buffer(required_bytes, 0)
    StrPut(json, json_utf8, "UTF-8")
    ran := DllCall(prepare_proc, "Ptr", json_utf8, "Int")
    if ran = 0
        return json
    pretty_json_utf8 := Buffer(ran, 0)
    DllCall(prettify_proc, "Ptr", json_utf8, "Ptr", pretty_json_utf8, "Int")
    name := StrGet(pretty_json_utf8, ran, "UTF-8")
    return name
}

OnClipboardChange clip_changed

clip_changed(clip_type) {
    if 1 = clip_type {
        prettify_with_clip_changed()
    }
}

prettify_with_clip_changed() {
    global prettify_proc
    str := A_Clipboard
    str := StrReplace(str, '`r`n')
    str := StrReplace(str, '`t')
    str := StrReplace(str, '\"', '"')
    str := StrReplace(str, '\\', '\')
    required_bytes := StrPut(str, "UTF-8")
    json_utf8 := Buffer(required_bytes, 0)
    StrPut(str, json_utf8, "UTF-8")
    ran := DllCall(prepare_proc, "Ptr", json_utf8, "Int")
    if ran > required_bytes {
        ToolTip "prettify json..."
        pretty_json_utf8 := Buffer(ran, 0)
        DllCall(prettify_proc, "Ptr", json_utf8, "Ptr", pretty_json_utf8, "Int")
        A_Clipboard := StrGet(pretty_json_utf8, ran, "UTF-8")
        ToolTip ; Turn off the tip.
    }
}
```
