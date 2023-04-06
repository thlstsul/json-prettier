; 格式化JSON字符串
!j:: {
    A_Clipboard := ""
    Send "^c"
    ClipWait
    A_Clipboard := pretty(A_Clipboard)
    Send "^v"
}

PJ_PATH := "json_prettier.dll"
JsonPrettier := DllCall("LoadLibrary", "Str", PJ_PATH, "Ptr")
prepare_proc := DllCall("GetProcAddress", "Ptr", JsonPrettier, "AStr", "prepare", "Ptr")
pretty_proc := DllCall("GetProcAddress", "Ptr", JsonPrettier, "AStr", "pretty", "Ptr")

pretty(json) {
    global pretty_proc
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
    DllCall(pretty_proc, "Ptr", json_utf8, "Ptr", pretty_json_utf8, "Int")
    name := StrGet(pretty_json_utf8, ran, "UTF-8")
    return name
}

OnClipboardChange clip_changed

clip_changed(clip_type) {
    if 1 = clip_type {
        pretty_with_clip_changed()
    }
}

pretty_with_clip_changed() {
    global pretty_proc
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
        ToolTip "Pretty json..."
        pretty_json_utf8 := Buffer(ran, 0)
        DllCall(pretty_proc, "Ptr", json_utf8, "Ptr", pretty_json_utf8, "Int")
        A_Clipboard := StrGet(pretty_json_utf8, ran, "UTF-8")
        Sleep(1000)
        ToolTip ; Turn off the tip.
    }
}
