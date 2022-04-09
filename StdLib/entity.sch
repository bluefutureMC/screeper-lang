#macro SELECTOR(varient, ...)
__EVAL(
`@`, varient, `[`,
#repeat ARG __ARGS
ifdef __FIRST
ARG,
#endif
#define __FIRST
#endrepeat
#undef __FIRST
`]`
)
#endmacro

#macro SELECT_PLAYERS(...)
SELECTOR(`a`
#repeat ARG __ARGS
,ARG
#endrepeat
)
#endmacro

#macro SELECT_ENTITIES(...)
SELECTOR(`e`
#repeat ARG __ARGS
,ARG
#endrepeat
)
#endmacro

#macro SUMMON(out, type, position)
write `summon `, type, ` `
#ifdef position
, position
#else
, `~ ~ ~`
#endif
, ` {Tags:[screeper-summon]}`;
write `data modify storage `, __STORAGE(out), ` `, out, ` set from entity @e[tag=screeper-summon,limit=1] UUID`;
write `tag @e[tag=screeper-summon] remove screeper-summon`;
#endmacro