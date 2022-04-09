#macro MODIFY_TARGET(_VAR_)
__MEM_TYPE(_VAR_), ` `, __MEM_LOCATION(_VAR_)
#endmacro

#macro MODIFY_FROM(_VAR_, _TARGET_, _PATH_)
write `data modify `, MODIFY_TARGET(_VAR_), ` `, _VAR_, ` set from `, _TARGET_, ` `, _PATH_;
#endmacro