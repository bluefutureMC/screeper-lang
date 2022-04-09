#macro M_CHAT(...)
write `tellraw @a [""`

#repeat ARG __ARGS

#if __TYPE(ARG) == "REF"
, `,{"storage":"`, __STORAGE(ARG), `","nbt":"`, ARG, `"}`
#else
, `,{"text":"`, ARG, `"}`
#endif

#endrepeat

`]`;
#endmacro

