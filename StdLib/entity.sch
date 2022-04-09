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

#macro SUMMON(out, type, position, nbt)
write `summon `, type, ` `
#ifdef position
, position
#else
, `~ ~ ~`
#endif
#ifdef nbt
, ` `, nbt
#endif
;
#endmacro