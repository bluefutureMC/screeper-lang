#macro M_CHAT(...)
write `tellraw @a [`

#repeat __ARG __ARGS

#if TOKEN_TYPE(__ARG) == "IDENTIFIER"
,`{"storage":"`,__STORAGE(__ARG),`","nbt":"`,__ARG,`"}`
#elif TOKEN_TYPE(__ARG) == "STRING_LITERAL"
,`{"text":"`,__ARG,`"}`
#else
,`{"text":"__ARG"}`
#endif

#endrepeat

`]`;
#endmacro

